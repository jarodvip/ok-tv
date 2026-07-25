use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;

static CATCHUP_TYPE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"catchup="(.?|.+?)""#).unwrap());
static CATCHUP_SOURCE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"catchup-source="(.?|.+?)""#).unwrap());
static CATCHUP_REPLACE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"catchup-replace="(.?|.+?)""#).unwrap());
static TVG_CHNO_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"tvg-chno="(.?|.+?)""#).unwrap());
static TVG_LOGO_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"tvg-logo="(.?|.+?)""#).unwrap());
static TVG_NAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"tvg-name="(.?|.+?)""#).unwrap());
static TVG_ID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"tvg-id="(.?|.+?)""#).unwrap());
static GROUP_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"group-title="(.?|.+?)""#).unwrap());
static NAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#",(.+?)$"#).unwrap());
static HTTP_UA_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"http-user-agent="(.?|.+?)""#).unwrap());

#[derive(Serialize, Default)]
struct RustChannel {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    urls: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    number: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    logo: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    epg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ua: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    click: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    format: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    origin: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    referer: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    tvg_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    tvg_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    catchup: Option<RustCatchup>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    header: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drm: Option<RustDrm>,
}

#[derive(Serialize, Default, Clone)]
struct RustCatchup {
    #[serde(skip_serializing_if = "String::is_empty")]
    r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    days: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    regex: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    source: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    replace: String,
}

#[derive(Serialize, Default)]
struct RustDrm {
    #[serde(skip_serializing_if = "String::is_empty")]
    key: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    r#type: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    force_key: bool,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    header: HashMap<String, String>,
}

#[derive(Serialize)]
struct RustGroup {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    channel: Vec<RustChannel>,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
}

struct Setting {
    ua: Option<String>,
    key: Option<String>,
    r#type: Option<String>,
    click: Option<String>,
    format: Option<String>,
    origin: Option<String>,
    referer: Option<String>,
    parse: Option<i32>,
    force_key: bool,
    header: HashMap<String, String>,
    drm_header: HashMap<String, String>,
}

impl Setting {
    fn new() -> Self {
        Self {
            ua: None,
            key: None,
            r#type: None,
            click: None,
            format: None,
            origin: None,
            referer: None,
            parse: None,
            force_key: false,
            header: HashMap::new(),
            drm_header: HashMap::new(),
        }
    }

    fn find(line: &str) -> bool {
        line.starts_with("ua")
            || line.starts_with("parse")
            || line.starts_with("click")
            || line.starts_with("header")
            || line.starts_with("format")
            || line.starts_with("origin")
            || line.starts_with("referer")
            || line.starts_with("forceKey")
            || line.starts_with("#EXTHTTP:")
            || line.starts_with("#EXTVLCOPT:")
            || line.starts_with("#KODIPROP:")
    }

    fn check(&mut self, line: &str) {
        if line.starts_with("ua") {
            self.ua(line);
        } else if line.starts_with("parse") {
            self.parse_setting(line);
        } else if line.starts_with("click") {
            self.click(line);
        } else if line.starts_with("header") {
            self.header(line);
        } else if line.starts_with("format") {
            self.format(line);
        } else if line.starts_with("origin") {
            self.origin(line);
        } else if line.starts_with("referer") {
            self.referer(line);
        } else if line.starts_with("#EXTHTTP:") {
            self.header(line);
        } else if line.starts_with("forceKey") {
            self.force_key(line);
        } else if line.starts_with("#EXTVLCOPT:http-origin") {
            self.origin(line);
        } else if line.starts_with("#EXTVLCOPT:http-user-agent") {
            self.ua(line);
        } else if line.starts_with("#EXTVLCOPT:http-referrer") {
            self.referrer(line);
        } else if line.starts_with("#KODIPROP:inputstream.adaptive.license_key") {
            self.key(line);
        } else if line.starts_with("#KODIPROP:inputstream.adaptive.license_type") {
            self.r#type(line);
        } else if line.starts_with("#KODIPROP:inputstream.adaptive.drm_legacy") {
            self.drm_legacy(line);
        } else if line.starts_with("#KODIPROP:inputstream.adaptive.manifest_type") {
            self.format(line);
        } else if line.starts_with("#KODIPROP:inputstream.adaptive.stream_headers")
            || line.starts_with("#KODIPROP:inputstream.adaptive.common_headers")
        {
            self.headers(line);
        }
    }

    fn copy_to_channel(&self, ch: &mut RustChannel) {
        if let Some(ref ua) = self.ua {
            ch.ua = ua.clone();
        }
        if let Some(ref parse) = self.parse {
            ch.parse = Some(*parse);
        }
        if let Some(ref click) = self.click {
            ch.click = click.clone();
        }
        if let Some(ref format) = self.format {
            ch.format = format.clone();
        }
        if let Some(ref origin) = self.origin {
            ch.origin = origin.clone();
        }
        if let Some(ref referer) = self.referer {
            ch.referer = referer.clone();
        }
        if !self.header.is_empty() {
            ch.header = self.header.clone();
        }
        if self.key.is_some() && self.r#type.is_some() {
            let mut drm = RustDrm::default();
            drm.key = self.key.clone().unwrap_or_default();
            drm.r#type = self.r#type.clone().unwrap_or_default();
            drm.force_key = self.force_key;
            drm.header = self.drm_header.clone();
            ch.drm = Some(drm);
        }
    }

    fn clear(&mut self) {
        self.ua = None;
        self.key = None;
        self.r#type = None;
        self.click = None;
        self.format = None;
        self.origin = None;
        self.referer = None;
        self.parse = None;
        self.force_key = false;
        self.header.clear();
        self.drm_header.clear();
    }

    fn ua(&mut self, line: &str) {
        if line.contains("user-agent=") {
            self.ua = extract_after(line, "user-agent=");
        } else if line.contains("ua=") {
            self.ua = extract_after(line, "ua=");
        } else if line.starts_with("ua ") {
            self.ua = extract_after(line, "ua ");
        }
    }

    fn referer(&mut self, line: &str) {
        self.referer = extract_after(line, "referer=");
    }

    fn referrer(&mut self, line: &str) {
        self.referer = extract_after(line, "referrer=");
    }

    fn parse_setting(&mut self, line: &str) {
        if let Some(val) = extract_after(line, "parse=") {
            if let Ok(n) = val.parse::<i32>() {
                self.parse = Some(n);
            }
        }
    }

    fn click(&mut self, line: &str) {
        self.click = extract_after(line, "click=");
    }

    fn format(&mut self, line: &str) {
        self.format = extract_after_any(line, &["format=", "manifest_type="]);
    }

    fn origin(&mut self, line: &str) {
        self.origin = extract_after(line, "origin=");
    }

    fn key(&mut self, line: &str) {
        self.key = extract_after_any(line, &["license_key="]);
        if self.key.is_none() || self.key.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
            self.key = Some(line.to_string());
        }
        if let Some(ref k) = self.key {
            if k.starts_with("http") {
                self.http_key();
            } else {
                self.local_key();
            }
        }
    }

    fn r#type(&mut self, line: &str) {
        self.r#type = extract_after_any(line, &["license_type="]);
        if self.r#type.is_none() || self.r#type.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
            self.r#type = Some(line.to_string());
        }
    }

    fn drm_legacy(&mut self, line: &str) {
        if let Some(val) = extract_after(line, "drm_legacy=") {
            let parts: Vec<&str> = val.split('|').collect();
            if !parts.is_empty() {
                self.r#type = Some(parts[0].trim().to_string());
            }
            if parts.len() > 1 {
                self.key = Some(parts[1].trim().to_string());
            }
        }
    }

    fn header(&mut self, line: &str) {
        let json_str = extract_after_any(line, &["#EXTHTTP:", "header="]);
        if let Some(s) = json_str {
            if let Ok(map) = serde_json::from_str::<HashMap<String, String>>(&s) {
                self.header.extend(map);
            }
        }
    }

    fn headers(&mut self, line: &str) {
        let entries: Vec<String> = if line.contains("headers=") {
            extract_after(line, "headers=")
                .map(|v| v.split('&').map(|s| s.to_string()).collect())
                .unwrap_or_default()
        } else if line.contains('|') {
            line.split('|').map(|s| s.to_string()).collect()
        } else {
            line.trim().split('&').map(|s| s.to_string()).collect()
        };
        for param in entries {
            self.parse_kv_into(&param);
        }
    }

    fn force_key(&mut self, line: &str) {
        self.force_key = extract_after(line, "forceKey=")
            .map(|s| s == "true")
            .unwrap_or(false);
    }

    fn http_key(&mut self) {
        if let Some(ref key) = self.key {
            if let Some((_, header_part)) = key.split_once('|') {
                self.drm_header = parse_kv_string(header_part);
            }
        }
    }

    fn local_key(&mut self) {
        if let Some(ref key) = self.key {
            self.key = Some(key.replace('"', "").replace('{', "").replace('}', ""));
        }
    }

    fn parse_kv_into(&mut self, param: &str) {
        if !param.contains('=') {
            return;
        }
        let parts: Vec<&str> = param.splitn(2, '=').collect();
        if parts.len() != 2 {
            return;
        }
        let k = parts[0].trim().replace('"', "");
        let v = parts[1].trim().replace('"', "");
        match k.as_str() {
            "drmScheme" => self.r#type = Some(v),
            "drmLicense" => self.key = Some(v),
            _ => { self.header.insert(k, v); }
        }
    }
}

fn extract_after(line: &str, keyword: &str) -> Option<String> {
    line.find(keyword)
        .map(|i| line[i + keyword.len()..].trim().trim_matches('"').to_string())
}

fn extract_after_any(line: &str, keywords: &[&str]) -> Option<String> {
    for kw in keywords {
        if line.contains(kw) {
            return extract_after(line, kw);
        }
    }
    None
}

fn parse_kv_string(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for param in s.split('&') {
        if let Some((k, v)) = param.split_once('=') {
            map.insert(k.trim().replace('"', ""), v.trim().replace('"', ""));
        }
    }
    map
}

fn ensure_group(groups: &mut Vec<RustGroup>, name: String) -> usize {
    if let Some((i, _)) = groups.iter_mut().enumerate().find(|(_, g)| g.name == name) {
        return i;
    }
    groups.push(RustGroup { name, channel: Vec::new() });
    groups.len() - 1
}

fn ensure_channel(group: &mut RustGroup, name: String) -> usize {
    if let Some((i, _)) = group.channel.iter().enumerate().find(|(_, c)| c.name == name && !name.is_empty()) {
        return i;
    }
    group.channel.push(RustChannel {
        name,
        ..Default::default()
    });
    group.channel.len() - 1
}

fn parse_m3u(text: &str) -> Vec<RustGroup> {
    let mut groups: Vec<RustGroup> = Vec::new();
    let mut global_catchup = RustCatchup::default();
    let mut setting = Setting::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if Setting::find(trimmed) {
            setting.check(trimmed);
            continue;
        }

        if trimmed.starts_with("#EXTM3U") {
            global_catchup.r#type = CATCHUP_TYPE_RE.captures(trimmed).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            global_catchup.source = CATCHUP_SOURCE_RE.captures(trimmed).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            global_catchup.replace = CATCHUP_REPLACE_RE.captures(trimmed).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            continue;
        }

        if trimmed.starts_with("#EXTINF:") {
            let group_name = GROUP_RE.captures(trimmed).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            let name = NAME_RE.captures(trimmed).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();

            let gi = ensure_group(&mut groups, group_name);
            let ci = ensure_channel(&mut groups[gi], name);
            let ch = &mut groups[gi].channel[ci];

            ch.ua = HTTP_UA_RE.captures(trimmed).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            ch.tvg_name = TVG_NAME_RE.captures(trimmed).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            ch.number = TVG_CHNO_RE.captures(trimmed).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            ch.logo = TVG_LOGO_RE.captures(trimmed).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            ch.tvg_id = TVG_ID_RE.captures(trimmed).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();

            let mut unknown = RustCatchup::default();
            unknown.r#type = CATCHUP_TYPE_RE.captures(trimmed).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            unknown.source = CATCHUP_SOURCE_RE.captures(trimmed).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            unknown.replace = CATCHUP_REPLACE_RE.captures(trimmed).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            ch.catchup = RustCatchup::decide(unknown, &global_catchup);
            continue;
        }

        if trimmed.starts_with("#") || !trimmed.contains("://") {
            continue;
        }

        if let Some(group) = groups.last_mut() {
            if let Some(ch) = group.channel.last_mut() {
                setting.copy_to_channel(ch);
                if let Some(pipe_pos) = trimmed.find('|') {
                    let url = trimmed[..pipe_pos].trim();
                    let header_str = &trimmed[pipe_pos + 1..];
                    for param in header_str.split('&') {
                        if param.contains('=') {
                            let kv: Vec<&str> = param.splitn(2, '=').collect();
                            if kv.len() == 2 {
                                ch.header.insert(
                                    kv[0].trim().replace('"', ""),
                                    kv[1].trim().replace('"', ""),
                                );
                            }
                        }
                    }
                    ch.urls.push(url.to_string());
                } else {
                    ch.urls.push(trimmed.to_string());
                }
                setting.clear();
            }
        }
    }

    groups
}

impl RustCatchup {
    fn decide(major: Self, minor: &Self) -> Option<Self> {
        if !major.source.is_empty() {
            return Some(major);
        }
        if !minor.source.is_empty() {
            return Some(minor.clone());
        }
        None
    }
}

fn parse_txt(text: &str) -> Vec<RustGroup> {
    let mut groups: Vec<RustGroup> = Vec::new();
    let mut setting = Setting::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if Setting::find(trimmed) {
            setting.check(trimmed);
            if trimmed.contains("#genre#") {
                setting.clear();
            }
            continue;
        }

        if trimmed.contains("#genre#") {
            let parts: Vec<&str> = trimmed.splitn(2, ',').collect();
            let group_name = parts[0].trim();
            if !group_name.is_empty() && !group_name.contains("#genre#") {
                groups.push(RustGroup {
                    name: group_name.to_string(),
                    channel: Vec::new(),
                });
            }
            setting.clear();
            continue;
        }

        let parts: Vec<&str> = trimmed.splitn(2, ',').collect();
        if parts.len() < 2 {
            continue;
        }

        let ch_name = parts[0].trim().to_string();
        let urls_str = parts[1].trim();

        if groups.is_empty() {
            groups.push(RustGroup {
                name: String::new(),
                channel: Vec::new(),
            });
        }

        let group = groups.last_mut().unwrap();
        let is_new = !ch_name.is_empty() && group.channel.last().map(|c| c.name != ch_name).unwrap_or(true);
        if is_new {
            group.channel.push(RustChannel {
                name: ch_name,
                ..Default::default()
            });
        }
        let channel = group.channel.last_mut().unwrap();

        for url in urls_str.split('#') {
            let url = url.trim();
            if !url.contains("://") {
                continue;
            }
            if let Some(pipe_pos) = url.find('|') {
                let header_str = &url[pipe_pos + 1..];
                for param in header_str.split('&') {
                    if param.contains('=') {
                        let kv: Vec<&str> = param.splitn(2, '=').collect();
                        if kv.len() == 2 {
                            channel.header.insert(
                                kv[0].trim().replace('"', ""),
                                kv[1].trim().replace('"', ""),
                            );
                        }
                    }
                }
                channel.urls.push(url[..pipe_pos].trim().to_string());
            } else {
                channel.urls.push(url.to_string());
            }
        }
        setting.copy_to_channel(channel);
    }

    groups
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_api_parser_RustParser_nativeInit(
    _env: JNIEnv,
    _class: JClass,
) {
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_api_parser_RustParser_nativeParse(
    mut env: JNIEnv,
    _class: JClass,
    text: JString,
) -> jstring {
    let result = parse_impl(&mut env, text);
    match result {
        Ok(s) => s,
        Err(_) => env.new_string("[]").unwrap().into_raw(),
    }
}

fn parse_impl(env: &mut JNIEnv, text: JString) -> Result<jstring, Box<dyn std::error::Error>> {
    let text_str: String = env.get_string(&text)?.into();
    let groups = if text_str.contains("#EXTM3U") {
        parse_m3u(&text_str)
    } else {
        parse_txt(&text_str)
    };
    let json = serde_json::to_string(&groups)?;
    let jstr = env.new_string(json)?;
    Ok(jstr.into_raw())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_m3u() {
        let input = r#"#EXTM3U
#EXTINF:-1 tvg-id="" tvg-name="CCTV1" tvg-logo="" group-title="央视频道",CCTV1
http://example.com/cctv1.m3u8
#EXTINF:-1 tvg-id="" tvg-name="CCTV2" tvg-logo="" group-title="央视频道",CCTV2
http://example.com/cctv2.m3u8|header1=val1&header2=val2
#EXTINF:-1 tvg-id="" tvg-name="湖南卫视" tvg-logo="" group-title="卫视频道",湖南卫视
http://example.com/hunan.m3u8"#;
        let groups = parse_m3u(input);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].name, "央视频道");
        assert_eq!(groups[0].channel.len(), 2);
        assert_eq!(groups[0].channel[0].name, "CCTV1");
        assert_eq!(groups[0].channel[0].urls[0], "http://example.com/cctv1.m3u8");
        assert_eq!(groups[0].channel[0].header.len(), 0);
        assert_eq!(groups[0].channel[1].name, "CCTV2");
        assert_eq!(groups[0].channel[1].header.get("header1").map(|s| s.as_str()), Some("val1"));
        assert_eq!(groups[1].name, "卫视频道");
        assert_eq!(groups[1].channel.len(), 1);
        assert_eq!(groups[1].channel[0].tvg_id, "");
        assert_eq!(groups[1].channel[0].tvg_name, "湖南卫视");
    }

    #[test]
    fn test_parse_m3u_with_catchup() {
        let input = r#"#EXTM3U catchup="append" catchup-source="?playseek=${(b)yyyyMMddHHmmss}-${(e)yyyyMMddHHmmss}"
#EXTINF:-1 group-title="test",Channel1
http://example.com/ch1.m3u8"#;
        let groups = parse_m3u(input);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].channel[0].catchup.as_ref().unwrap().r#type, "append");
        assert_eq!(groups[0].channel[0].catchup.as_ref().unwrap().source, "?playseek=${(b)yyyyMMddHHmmss}-${(e)yyyyMMddHHmmss}");
    }

    #[test]
    fn test_parse_txt_genre_groups() {
        let input = "央视频道,#genre#\nCCTV1,http://example.com/cctv1.m3u8#http://example.com/cctv1b.m3u8\n卫视频道,#genre#\n湖南卫视,http://example.com/hunan.m3u8|User-Agent=TestAgent\n";
        let groups = parse_txt(input);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].name, "央视频道");
        assert_eq!(groups[0].channel.len(), 1);
        assert_eq!(groups[0].channel[0].name, "CCTV1");
        assert_eq!(groups[0].channel[0].urls.len(), 2);
        assert_eq!(groups[0].channel[0].urls[0], "http://example.com/cctv1.m3u8");
        assert_eq!(groups[0].channel[0].ua, "");
        assert_eq!(groups[1].name, "卫视频道");
        assert_eq!(groups[1].channel[0].name, "湖南卫视");
        assert_eq!(groups[1].channel[0].header.get("User-Agent").map(|s| s.as_str()), Some("TestAgent"));
    }

    #[test]
    fn test_parse_txt_no_genre() {
        let input = "CCTV1,http://example.com/cctv1.m3u8\nCCTV2,http://example.com/cctv2.m3u8\n";
        let groups = parse_txt(input);
        assert_eq!(groups.len(), 1);
        assert!(groups[0].name.is_empty());
        assert_eq!(groups[0].channel.len(), 2);
    }

    #[test]
    fn test_parse_skip_empty() {
        let input = r#"#EXTM3U

#EXTINF:-1 group-title="test",Chan
http://example.com/ch.m3u8

#EXTINF:-1 group-title="test",Chan2
http://example.com/ch2.m3u8"#;
        let groups = parse_m3u(input);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].channel.len(), 2);
    }
}
