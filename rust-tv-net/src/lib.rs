use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jstring};
use jni::{JNIEnv, JavaVM};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use thiserror::Error;

static JAVA_VM: OnceCell<Mutex<Option<JavaVM>>> = OnceCell::new();
static RULES: OnceCell<Mutex<Option<NetRules>>> = OnceCell::new();

fn init_state() {
    JAVA_VM.get_or_init(|| Mutex::new(None));
    RULES.get_or_init(|| Mutex::new(None));
}

fn rules() -> Option<NetRules> {
    RULES.get()?.lock().clone()
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NetRules {
    pub proxies: Vec<ProxyRule>,
    pub headers: Vec<HeaderRule>,
    pub ads: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyRule {
    pub host: String,
    pub proxy_type: String,
    pub hostname: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderRule {
    pub host: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum NetError {
    #[error("invalid rule json: {0}")]
    InvalidRule(String),
    #[error("java vm not attached")]
    JavaVmMissing,
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_net_RustNet_nativeInit(
    mut env: JNIEnv,
    _class: JClass,
    rules_json: JString,
) -> jstring {
    init_state();
    *JAVA_VM.get().unwrap().lock() = env.get_java_vm().ok();

    let text: String = match env.get_string(&rules_json) {
        Ok(text) => text.into(),
        Err(err) => return throw_and_empty_string(&mut env, NetError::InvalidRule(err.to_string())),
    };

    match serde_json::from_str::<NetRules>(&text) {
        Ok(rules) => {
            *RULES.get().unwrap().lock() = Some(rules);
            empty_string(&mut env)
        }
        Err(err) => throw_and_empty_string(&mut env, NetError::InvalidRule(err.to_string())),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_net_RustNet_nativeInit(
    env: JNIEnv,
    _class: JClass,
    rules_json: JString,
) -> jstring {
    Java_com_fongmi_android_tv_net_RustNet_nativeInit(env, _class, rules_json)
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_net_RustNet_nativeResolveProxy(
    mut env: JNIEnv,
    _class: JClass,
    host: JString,
) -> jstring {
    let host_text: String = match env.get_string(&host) {
        Ok(text) => text.into(),
        Err(err) => return throw_and_empty_string(&mut env, NetError::InvalidRule(err.to_string())),
    };

    let rule = match rules() {
        Some(rules) => resolve_proxy(rules, &host_text),
        None => None,
    };

    match rule {
        Some(value) => to_json_string(&mut env, &value).unwrap_or_else(|err| throw_and_empty_string(&mut env, err)),
        None => empty_string(&mut env),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_net_RustNet_nativeResolveProxy(
    env: JNIEnv,
    _class: JClass,
    host: JString,
) -> jstring {
    Java_com_fongmi_android_tv_net_RustNet_nativeResolveProxy(env, _class, host)
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_net_RustNet_nativeShouldBlock(
    mut env: JNIEnv,
    _class: JClass,
    url: JString,
) -> jboolean {
    let url_text: String = match env.get_string(&url) {
        Ok(text) => text.into(),
        Err(_) => return jni::sys::JNI_FALSE,
    };

    match rules() {
        Some(rules) => should_block(rules, &url_text),
        None => jni::sys::JNI_FALSE,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_net_RustNet_nativeShouldBlock(
    mut env: JNIEnv,
    _class: JClass,
    url: JString,
) -> jboolean {
    Java_com_fongmi_android_tv_net_RustNet_nativeShouldBlock(env, _class, url)
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_net_RustNet_nativeInjectHeaders(
    mut env: JNIEnv,
    _class: JClass,
    host: JString,
    headers_json: JString,
) -> jstring {
    let host_text: String = match env.get_string(&host) {
        Ok(text) => text.into(),
        Err(err) => return throw_and_empty_string(&mut env, NetError::InvalidRule(err.to_string())),
    };

    let headers_text: String = match env.get_string(&headers_json) {
        Ok(text) => text.into(),
        Err(err) => return throw_and_empty_string(&mut env, NetError::InvalidRule(err.to_string())),
    };

    let mut headers: HashMap<String, String> = match serde_json::from_str(&headers_text) {
        Ok(map) => map,
        Err(err) => return throw_and_empty_string(&mut env, NetError::InvalidRule(err.to_string())),
    };

    match rules() {
        Some(rules) => inject_headers(rules, &host_text, &mut headers),
        None => {}
    }

    match serde_json::to_string(&headers) {
        Ok(json) => match env.new_string(json) {
            Ok(value) => value.into_raw(),
            Err(err) => throw_and_empty_string(&mut env, NetError::InvalidRule(err.to_string())),
        },
        Err(err) => throw_and_empty_string(&mut env, NetError::InvalidRule(err.to_string())),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_net_RustNet_nativeInjectHeaders(
    mut env: JNIEnv,
    _class: JClass,
    host: JString,
    headers_json: JString,
) -> jstring {
    Java_com_fongmi_android_tv_net_RustNet_nativeInjectHeaders(env, _class, host, headers_json)
}

fn resolve_proxy(rules: NetRules, host: &str) -> Option<ProxyRule> {
    rules.proxies.iter().find(|rule| host_matches(&rule.host, host)).cloned()
}

fn should_block(rules: NetRules, url: &str) -> jboolean {
    let host = url.split("://")
        .nth(1)
        .and_then(|part| part.split('/').next())
        .unwrap_or(url);

    for ad in &rules.ads {
        if host_matches(ad, host) {
            return jni::sys::JNI_TRUE;
        }
    }

    jni::sys::JNI_FALSE
}

fn inject_headers(rules: NetRules, host: &str, headers: &mut HashMap<String, String>) {
    for rule in &rules.headers {
        if host_matches(&rule.host, host) {
            for (key, value) in &rule.headers {
                if !key.is_empty() {
                    headers.insert(key.clone(), value.clone());
                }
            }
        }
    }
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

    if let Some(stripped) = normalized_pattern.strip_prefix("*.") {
        if normalized_host == stripped || normalized_host.ends_with(&(".".to_string() + stripped)) {
            return true;
        }
    }

    if normalized_host.ends_with(&(".".to_string() + normalized_pattern)) {
        return true;
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

fn to_json_string<T: Serialize>(env: &mut JNIEnv, value: &T) -> Result<jstring, NetError> {
    let text = serde_json::to_string(value).map_err(|err| NetError::InvalidRule(err.to_string()))?;
    Ok(env.new_string(text).map_err(|err| NetError::InvalidRule(err.to_string()))?.into_raw())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_matches_exact_and_suffix() {
        assert!(host_matches("example.com", "example.com"));
        assert!(host_matches("example.com", "www.example.com"));
        assert!(host_matches("*.example.com", "www.example.com"));
        assert!(!host_matches("other.com", "example.com"));
    }

    #[test]
    fn rules_round_trip_and_match() {
        let rules = NetRules {
            proxies: vec![ProxyRule { host: "api.example.com".into(), proxy_type: "http".into(), hostname: "127.0.0.1".into(), port: 8080 }],
            headers: vec![HeaderRule { host: ".example.com".into(), headers: HashMap::from([("X-Proxy".into(), "rust".into())]) }],
            ads: vec!["ads.example.com".into(), "tracker.example.com".into()],
        };

        let json = serde_json::to_string(&rules).unwrap();
        let parsed: NetRules = serde_json::from_str(&json).unwrap();
        assert!(should_block(parsed.clone(), "https://ads.example.com/path") == jni::sys::JNI_TRUE);
        assert!(should_block(parsed.clone(), "https://tracker.example.com/x") == jni::sys::JNI_TRUE);
        assert!(should_block(parsed.clone(), "https://example.com/x") == jni::sys::JNI_FALSE);

        let mut headers = HashMap::from([("Accept".into(), "*/*".into())]);
        inject_headers(parsed.clone(), "www.example.com", &mut headers);
        assert_eq!(headers.get("X-Proxy"), Some(&"rust".into()));

        let proxy = resolve_proxy(parsed.clone(), "www.example.com");
        assert!(proxy.is_none());

        let proxy = resolve_proxy(parsed, "api.example.com");
        assert_eq!(proxy.as_ref().map(|p| p.hostname.as_str()), Some("127.0.0.1"));
    }
}
