use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::{JNIEnv, JavaVM};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;
use url::Url;

static mut JAVA_VM: Option<JavaVM> = None;
static mut CONFIG: Option<DnsConfig> = None;
static mut CACHE: Option<HostCache> = None;

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
    unsafe {
        JAVA_VM = env.get_java_vm().ok();
    }

    let text: String = match env.get_string(&config_json) {
        Ok(text) => text.into(),
        Err(err) => return throw_and_empty_string(&mut env, DnsError::InvalidConfig(err.to_string())),
    };

    match serde_json::from_str::<DnsConfig>(&text) {
        Ok(config) => {
            let ttl_secs = config.ttl_secs;
            unsafe {
                CONFIG = Some(config);
                CACHE = Some(HostCache::new(ttl_secs));
            }
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
    let host_text: String = match env.get_string(&host) {
        Ok(text) => text.into(),
        Err(err) => return throw_and_empty_string(&mut env, DnsError::InvalidConfig(err.to_string())),
    };

    match unsafe { CACHE.as_mut() } {
        Some(cache) => match cache.get(&host_text) {
            Some(ip) => to_json_string(&mut env, &ResolveResult::cached(ip)).unwrap_or_else(|err| throw_and_empty_string(&mut env, err)),
            None => {
                let result = resolve_host(&host_text);
                cache.insert(&host_text, result.ip.clone());
                to_json_string(&mut env, &result).unwrap_or_else(|err| throw_and_empty_string(&mut env, err))
            }
        },
        None => empty_string(&mut env),
    }
}

fn resolve_host(host: &str) -> ResolveResult {
    if let Some(rule) = unsafe { CONFIG.as_ref() }.and_then(|config| match_host_rule(config, host)) {
        return ResolveResult { ip: rule.target, source: "hosts".into(), ttl_secs: unsafe { CONFIG.as_ref() }.map(|c| c.ttl_secs).unwrap_or(0) };
    }

    match resolve_with_doh(host) {
        Ok(ip) => ResolveResult { ip, source: "doh".into(), ttl_secs: unsafe { CONFIG.as_ref() }.map(|c| c.ttl_secs).unwrap_or(0) },
        Err(_) => ResolveResult { ip: host.into(), source: "passthrough".into(), ttl_secs: 0 },
    }
}

fn match_host_rule(config: &DnsConfig, host: &str) -> Option<HostRule> {
    for rule in &config.hosts {
        if host_matches(&rule.host, host) {
            return Some(rule.clone());
        }
    }
    None
}

fn resolve_with_doh(host: &str) -> Result<String, DnsError> {
    let urls = match doh_urls() {
        Some(urls) => urls,
        None => return Err(DnsError::DohRequest("no doh".into())),
    };

    let mut last_err = None;
    for doh_url in urls {
        let request_url = match doh_url.join(&format!("?name={}&type=A", urlencoding::encode(host))) {
            Ok(url) => url,
            Err(err) => {
                last_err = Some(err.to_string());
                continue;
            }
        };
        let client = match reqwest::blocking::Client::builder().user_agent("ok-tv-dns/0.1").timeout(Duration::from_secs(3)).build() {
            Ok(client) => client,
            Err(err) => {
                last_err = Some(err.to_string());
                continue;
            }
        };
        let response = match client.get(request_url).header("Accept", "application/dns-json").send() {
            Ok(resp) => resp,
            Err(err) => {
                last_err = Some(err.to_string());
                continue;
            }
        };
        if !response.status().is_success() {
            last_err = Some(format!("doh status={}", response.status()));
            continue;
        }
        match response.json::<DohResponse>() {
            Ok(doh_response) => {
                if let Some(ip) = doh_response.first_ip().map(|ip| ip.to_string()) {
                    return Ok(ip);
                }
                last_err = Some("doh response had no answer".to_string());
            }
            Err(err) => last_err = Some(err.to_string()),
        }
    }

    Err(DnsError::DohRequest(last_err.unwrap_or_else(|| "no answer".into())))
}

fn doh_urls() -> Option<Vec<Url>> {
    let config = unsafe { CONFIG.as_ref() }?;
    let urls = config.doh.iter().filter(|item| !item.url.is_empty()).filter_map(|item| Url::parse(&item.url).ok()).collect::<Vec<_>>();
    if urls.is_empty() { None } else { Some(urls) }
}

fn host_matches(pattern: &str, host: &str) -> bool {
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
    inner: HashMap<String, (String, u64)>,
}

impl HostCache {
    fn new(max_ttl: u64) -> Self {
        Self { max_ttl, inner: HashMap::new() }
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
    fn cache_insert_and_get() {
        let mut cache = HostCache::new(60);
        cache.insert("cache.example.com", "1.1.1.1".into());
        assert_eq!(cache.get("cache.example.com"), Some("1.1.1.1".to_string()));
    }

    #[test]
    fn host_rules_and_cache_flow() {
        let config = DnsConfig {
            doh: vec![],
            hosts: vec![
                HostRule { host: "cache.example.com".into(), target: "1.1.1.1".into() },
                HostRule { host: ".example.com".into(), target: "2.2.2.2".into() },
                HostRule { host: "*.example.org".into(), target: "3.3.3.3".into() },
            ],
            ttl_secs: 60,
        };

        let result = resolve_host_with_config(&config, "cache.example.com");
        assert_eq!(result.ip, "1.1.1.1");
        assert_eq!(result.source, "hosts");

        let result = resolve_host_with_config(&config, "www.example.com");
        assert_eq!(result.ip, "2.2.2.2");
        assert_eq!(result.source, "hosts");

        let result = resolve_host_with_config(&config, "a.example.org");
        assert_eq!(result.ip, "3.3.3.3");
        assert_eq!(result.source, "hosts");
    }

    #[test]
    fn passthrough_when_no_match() {
        let config = DnsConfig { doh: vec![], hosts: vec![], ttl_secs: 0 };
        let result = resolve_host_with_config(&config, "unknown.example");
        assert_eq!(result.ip, "unknown.example");
        assert_eq!(result.source, "passthrough");
    }

    fn resolve_host_with_config(config: &DnsConfig, host: &str) -> ResolveResult {
        unsafe { CONFIG = Some(config.clone()); }
        unsafe { CACHE = Some(HostCache::new(config.ttl_secs)); }
        resolve_host(host)
    }
}
