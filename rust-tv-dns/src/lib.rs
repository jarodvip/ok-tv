use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::{JNIEnv, JavaVM};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use once_cell::sync::OnceCell;
use thiserror::Error;

static JAVA_VM: OnceCell<Mutex<Option<JavaVM>>> = OnceCell::new();
static DNS_CONFIG: OnceCell<Mutex<Option<DnsConfig>>> = OnceCell::new();
static DNS_CACHE: OnceCell<Mutex<Option<HostCache>>> = OnceCell::new();

fn init_state() {
    JAVA_VM.get_or_init(|| Mutex::new(None));
    DNS_CONFIG.get_or_init(|| Mutex::new(None));
    DNS_CACHE.get_or_init(|| Mutex::new(None));
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DnsConfig {
    pub doh: Vec<DohRule>,
    pub hosts: Vec<HostRule>,
    pub ttl_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DohRule {
    pub name: String,
    pub url: String,
    pub ips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostRule {
    pub host: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveResult {
    pub ip: String,
    pub source: String,
    pub ttl_secs: u64,
}

impl ResolveResult {
    pub fn cached(ip: String) -> Self {
        Self { ip, source: "cache".into(), ttl_secs: 0 }
    }
}

#[derive(Debug, Error)]
pub enum DnsError {
    #[error("invalid dns config: {0}")]
    InvalidConfig(String),
    #[error("doh request failed: {0}")]
    DohRequest(String),
    #[error("java vm not attached")]
    JavaVmMissing,
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_net_RustDns_nativeInit(
    mut env: JNIEnv,
    _class: JClass,
    config_json: JString,
) -> jstring {
    init_state();

    let vm = env.get_java_vm().ok();

    let text: String = match env.get_string(&config_json) {
        Ok(text) => text.into(),
        Err(err) => return throw_and_empty_string(&mut env, DnsError::InvalidConfig(err.to_string())),
    };

    match serde_json::from_str::<DnsConfig>(&text) {
        Ok(config) => {
            let ttl_secs = config.ttl_secs;
            if let Some(vm) = vm {
                let _ = JAVA_VM.get().unwrap().lock().replace(vm);
            }
            let _ = DNS_CONFIG.get().unwrap().lock().replace(config);
            let _ = DNS_CACHE.get().unwrap().lock().replace(HostCache::new(ttl_secs));
            empty_string(&mut env)
        }
        Err(err) => throw_and_empty_string(&mut env, DnsError::InvalidConfig(err.to_string())),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_net_RustDns_nativeResolveHost(
    mut env: JNIEnv,
    _class: JClass,
    host: JString,
) -> jstring {
    init_state();

    let host_text: String = match env.get_string(&host) {
        Ok(text) => text.into(),
        Err(err) => return throw_and_empty_string(&mut env, DnsError::InvalidConfig(err.to_string())),
    };

    let result = match resolve_host(&host_text) {
        Ok(r) => r,
        Err(err) => return throw_and_empty_string(&mut env, err),
    };

    to_json_string(&mut env, &result).unwrap_or_else(|err| throw_and_empty_string(&mut env, err))
}

fn resolve_host(host: &str) -> Result<ResolveResult, DnsError> {
    let config_clone = DNS_CONFIG.get().unwrap().lock().as_ref().map(|c| c.clone());
    let config = match config_clone {
        Some(c) => c,
        None => return Ok(ResolveResult { ip: host.into(), source: "passthrough".into(), ttl_secs: 0 }),
    };

    {
        let mut cache_guard = DNS_CACHE.get().unwrap().lock();
        if let Some(cache) = cache_guard.as_mut() {
            if let Some(ip) = cache.get(host) {
                return Ok(ResolveResult::cached(ip));
            }
        }
    }

    if let Some(rule) = match_host_rule(&config, host) {
        return Ok(ResolveResult { ip: rule.target, source: "hosts".into(), ttl_secs: config.ttl_secs });
    }

    match resolve_with_doh_blocking(host) {
        Ok(ip) => Ok(ResolveResult { ip, source: "doh".into(), ttl_secs: config.ttl_secs }),
        Err(err) => {
            eprintln!("rust dns doh failed for {}: {}", host, err);
            Ok(ResolveResult { ip: host.into(), source: "passthrough".into(), ttl_secs: 0 })
        }
    }
}

fn match_host_rule(config: &DnsConfig, host: &str) -> Option<HostRule> {
    for rule in &config.hosts {
        if wildmatch_match(&rule.host, host) {
            return Some(rule.clone());
        }
    }
    None
}

fn resolve_with_doh_blocking(host: &str) -> Result<String, DnsError> {
    let urls = {
        let guard = DNS_CONFIG.get().unwrap().lock();
        match guard.as_ref() {
            Some(c) => c.doh.iter().filter(|item| !item.url.is_empty()).filter_map(|item| url::Url::parse(&item.url).ok()).collect::<Vec<_>>(),
            None => return Err(DnsError::DohRequest("no config".into())),
        }
    };

    if urls.is_empty() {
        return Err(DnsError::DohRequest("no doh".into()));
    }

    let client = match reqwest::blocking::Client::builder()
        .user_agent("ok-tv-dns/0.1")
        .timeout(std::time::Duration::from_secs(3))
        .build()
    {
        Ok(c) => c,
        Err(err) => return Err(DnsError::DohRequest(err.to_string())),
    };

    let mut last_err = None;
    for doh_url in &urls {
        let query = format!("name={}&type=A", urlencoding::encode(host));
        let request_url = match doh_url.join(&format!("?{}", query)) {
            Ok(url) => url,
            Err(err) => {
                last_err = Some(err.to_string());
                continue;
            }
        };
        match client.get(request_url).header("Accept", "application/dns-json").send() {
            Ok(resp) if resp.status().is_success() => match resp.json::<DohResponse>() {
                Ok(doh_response) => {
                    if let Some(ip) = doh_response.first_ip().map(|ip| ip.to_string()) {
                        return Ok(ip);
                    }
                    last_err = Some("doh response had no answer".into());
                }
                Err(err) => last_err = Some(err.to_string()),
            },
            Ok(resp) => {
                last_err = Some(format!("doh status={}", resp.status()));
            }
            Err(err) => {
                last_err = Some(err.to_string());
            }
        }
    }

    Err(DnsError::DohRequest(last_err.unwrap_or_else(|| "no answer".into())))
}

fn wildmatch_match(pattern: &str, host: &str) -> bool {
    if pattern.is_empty() {
        return false;
    }
    let normalized_pattern = pattern.trim().trim_start_matches('.');
    let normalized_host = host.trim().to_lowercase();

    if normalized_pattern == normalized_host {
        return true;
    }
    if normalized_host.ends_with(&(".".to_string() + normalized_pattern)) {
        return true;
    }
    if let Some(stripped) = normalized_pattern.strip_prefix("*.") {
        if normalized_host == stripped || normalized_host.ends_with(&(".".to_string() + stripped)) {
            return true;
        }
    }
    false
}

fn empty_string(env: &mut JNIEnv) -> jstring {
    env.new_string("").unwrap().into_raw()
}

fn throw_and_empty_string(env: &mut JNIEnv, err: impl std::fmt::Display) -> jstring {
    let _ = env.throw(err.to_string());
    env.new_string("").unwrap().into_raw()
}

fn to_json_string<T: Serialize>(env: &mut JNIEnv, value: &T) -> Result<jstring, DnsError> {
    let text = serde_json::to_string(value).map_err(|err| DnsError::InvalidConfig(err.to_string()))?;
    Ok(env.new_string(text).map_err(|err| DnsError::InvalidConfig(err.to_string()))?.into_raw())
}

#[derive(Debug, Deserialize)]
struct DohResponse {
    #[serde(rename = "Answer")]
    answer: Option<Vec<DohAnswer>>,
}

#[derive(Debug, Deserialize)]
struct DohAnswer {
    data: String,
}

impl DohResponse {
    fn first_ip(&self) -> Option<&String> {
        self.answer.as_ref()?.first().map(|item| &item.data)
    }
}

#[derive(Debug, Default)]
struct HostCache {
    max_ttl: u64,
    inner: std::collections::HashMap<String, (String, u64)>,
}

impl HostCache {
    fn new(max_ttl: u64) -> Self {
        Self { max_ttl, inner: std::collections::HashMap::new() }
    }

    fn get(&mut self, host: &str) -> Option<String> {
        let now = now_secs();
        self.inner.retain(|_, (_, expiry)| *expiry > now);
        self.inner.get(host).and_then(|(ip, expiry)| if *expiry > now { Some(ip.clone()) } else { None })
    }

    fn insert(&mut self, host: &str, ip: String) {
        if self.max_ttl == 0 {
            return;
        }
        self.inner.insert(host.into(), (ip, now_secs() + self.max_ttl));
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}

mod urlencoding {
    pub fn encode(input: &str) -> String {
        input.chars().fold(String::new(), |mut acc, ch| {
            match ch {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => acc.push(ch),
                _ => acc.push_str(&format!("%{:02X}", ch as u8)),
            }
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildmatch_same_as_tvnet() {
        assert!(wildmatch_match("example.com", "example.com"));
        assert!(wildmatch_match("example.com", "www.example.com"));
        assert!(wildmatch_match("*.example.com", "www.example.com"));
        assert!(wildmatch_match(".example.com", "www.example.com"));
        assert!(!wildmatch_match("other.com", "example.com"));
    }
}
