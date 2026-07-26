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
static DNS_CLIENT: OnceCell<Mutex<Option<reqwest::blocking::Client>>> = OnceCell::new();

fn init_state() {
    JAVA_VM.get_or_init(|| Mutex::new(None));
    DNS_CONFIG.get_or_init(|| Mutex::new(None));
    DNS_CACHE.get_or_init(|| Mutex::new(None));
    DNS_CLIENT.get_or_init(|| Mutex::new(None));
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
            let _ = DNS_CLIENT.get().unwrap().lock().replace(
                reqwest::blocking::Client::builder()
                    .user_agent("ok-tv-dns/0.1")
                    .timeout(std::time::Duration::from_secs(3))
                    .build()
                    .expect("failed to build DNS client"));
            empty_string(&mut env)
        }
        Err(err) => throw_and_empty_string(&mut env, DnsError::InvalidConfig(err.to_string())),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_net_RustDns_nativeInit(
    env: JNIEnv,
    _class: JClass,
    config_json: JString,
) -> jstring {
    Java_com_fongmi_android_tv_net_RustDns_nativeInit(env, _class, config_json)
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

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_net_RustDns_nativeResolveHost(
    env: JNIEnv,
    _class: JClass,
    host: JString,
) -> jstring {
    Java_com_fongmi_android_tv_net_RustDns_nativeResolveHost(env, _class, host)
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
        Ok(ip) => {
            let mut cache_guard = DNS_CACHE.get().unwrap().lock();
            if let Some(cache) = cache_guard.as_mut() {
                cache.insert(host.to_string(), ip.clone(), config.ttl_secs);
            }
            Ok(ResolveResult { ip, source: "doh".into(), ttl_secs: config.ttl_secs })
        }
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

fn is_safe_doh_url(url: &url::Url) -> bool {
    url.scheme() == "https" && !is_private_host(url.host_str())
}

fn is_private_host(host: Option<&str>) -> bool {
    let Some(host) = host else { return true; };
    // Reject cloud metadata, localhost, link-local, private ranges
    if host == "localhost" || host.ends_with(".localhost") { return true; }
    if host == "::1" || host.starts_with("fe80:") || host.starts_with("::ffff:") { return true; }
    // 10.x.x.x
    if host.starts_with("10.") { return true; }
    // 172.16-31.x.x
    if let Some(rest) = host.strip_prefix("172.") {
        if let Ok(octet) = rest.split('.').next().unwrap_or("").parse::<u16>() {
            if (16..=31).contains(&octet) { return true; }
        }
    }
    // 192.168.x.x
    if host.starts_with("192.168.") { return true; }
    // 169.254.x.x (link-local)
    if host.starts_with("169.254.") { return true; }
    // 127.x.x.x
    if host.starts_with("127.") { return true; }
    false
}

fn resolve_with_doh_blocking(host: &str) -> Result<String, DnsError> {
    let urls = {
        let guard = DNS_CONFIG.get().unwrap().lock();
        match guard.as_ref() {
            Some(c) => c.doh.iter()
                .filter(|item| !item.url.is_empty())
                .filter_map(|item| url::Url::parse(&item.url).ok())
                .filter(|u| is_safe_doh_url(u))
                .collect::<Vec<_>>(),
            None => return Err(DnsError::DohRequest("no config".into())),
        }
    };

    if urls.is_empty() {
        return Err(DnsError::DohRequest("no doh".into()));
    }

    let client = {
        let guard = DNS_CLIENT.get().unwrap().lock();
        match guard.as_ref() {
            Some(c) => c.clone(),
            None => return Err(DnsError::DohRequest("no client".into())),
        }
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
                    if doh_response.status != 0 {
                        last_err = Some(format!("doh status={}", doh_response.status));
                        continue;
                    }
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
    let normalized_pattern = pattern.trim().trim_start_matches('.').to_lowercase();
    let normalized_host = host.trim().to_lowercase();

    if normalized_pattern == normalized_host {
        return true;
    }
    if normalized_host.ends_with(&(".".to_string() + &normalized_pattern)) {
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
    #[serde(rename = "Status")]
    status: i32,
    #[serde(rename = "Answer")]
    answer: Option<Vec<DohAnswer>>,
}

#[derive(Debug, Deserialize)]
struct DohAnswer {
    #[serde(default)]
    data: String,
}

impl DohResponse {
    fn first_ip(&self) -> Option<&String> {
        self.answer.as_ref()?.first().filter(|a| is_valid_ip(&a.data)).map(|a| &a.data)
    }
}

fn is_valid_ip(s: &str) -> bool {
    if s.bytes().all(|b| b.is_ascii_digit() || b == b'.') {
        let parts: Vec<&str> = s.split('.').collect();
        return parts.len() == 4 && parts.iter().all(|p| {
            p.len() >= 1 && p.len() <= 3 && p.parse::<u16>().is_ok_and(|v| v <= 255)
        });
    }
    s.contains(':') && s.bytes().all(|b| b.is_ascii_hexdigit() || b == b':')
}

#[derive(Debug, Default)]
struct HostCache {
    inner: std::collections::HashMap<String, (String, u64)>,
}

impl HostCache {
    fn new(max_ttl: u64) -> Self {
        let _ = max_ttl; // reserved for future use
        Self { inner: std::collections::HashMap::new() }
    }

    fn get(&mut self, host: &str) -> Option<String> {
        let now = now_secs();
        self.inner.retain(|_, (_, expiry)| *expiry > now);
        self.inner.get(host).and_then(|(ip, expiry)| if *expiry > now { Some(ip.clone()) } else { None })
    }

    fn insert(&mut self, host: String, ip: String, ttl_secs: u64) {
        let expiry = now_secs().saturating_add(ttl_secs);
        self.inner.insert(host, (ip, expiry));
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()
}

mod urlencoding {
    pub fn encode(input: &str) -> String {
        const HEX: [u8; 16] = *b"0123456789ABCDEF";
        let bytes = input.as_bytes();
        let mut out = Vec::with_capacity(bytes.len() * 3);
        for &b in bytes {
            match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(b),
                _ => {
                    out.push(b'%');
                    out.push(HEX[(b >> 4) as usize]);
                    out.push(HEX[(b & 0xF) as usize]);
                }
            }
        }
        unsafe {
            debug_assert!(std::str::from_utf8(&out).is_ok());
            String::from_utf8_unchecked(out)
        }
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

    #[test]
    fn test_urlencoding_ascii() {
        assert_eq!(urlencoding::encode("hello"), "hello");
        assert_eq!(urlencoding::encode("a b"), "a%20b");
    }

    #[test]
    fn test_urlencoding_chinese() {
        // 中文 UTF-8: 中=E4 B8 AD 文=E6 96 87
        assert_eq!(urlencoding::encode("中文"), "%E4%B8%AD%E6%96%87");
    }

    #[test]
    fn test_urlencoding_special_chars() {
        assert_eq!(urlencoding::encode("a?b&c"), "a%3Fb%26c");
        assert_eq!(urlencoding::encode("test@host.com"), "test%40host.com");
    }
}
