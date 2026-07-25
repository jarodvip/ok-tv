use base64::{engine::general_purpose, Engine as _};
use axum::http::{Response, StatusCode, HeaderValue};
use jni::JavaVM;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::objects::JValueGen;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::LazyLock;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

static SERVER_CELL: LazyLock<Mutex<Option<RustServerHandle>>> = LazyLock::new(|| Mutex::new(None));
#[allow(static_mut_refs)]
static mut JAVA_VM: Option<JavaVM> = None;

#[allow(static_mut_refs)]
fn java_vm_ref() -> Option<&'static JavaVM> {
    unsafe { JAVA_VM.as_ref() }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("server already started")]
    AlreadyStarted,
    #[error("start failed: {0}")]
    StartFailed(String),
    #[error("invalid port: {0}")]
    InvalidPort(String),
}

pub struct RequestView<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub query: &'a str,
    pub body: &'a [u8],
    pub headers: &'a HashMap<String, String>,
}

pub struct ResponseView {
    pub status: u16,
    pub body: Vec<u8>,
    pub content_type: String,
}

impl ResponseView {
    pub fn ok_text(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 200,
            body: body.into(),
            content_type: "text/plain".to_string(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: 404,
            body: b"not found".to_vec(),
            content_type: "text/plain".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port_start: u16,
    pub port_end: u16,
}

#[derive(Debug, Clone)]
pub struct RustServerHandle {
    pub port: u16,
    handle: Arc<JoinHandle<()>>,
    shutdown: Arc<AtomicBool>,
}

impl RustServerHandle {
    pub async fn stop(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
        self.handle.abort();
    }
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_server_RustServer_nativeStart<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    port_start: jni::sys::jint,
    port_end: jni::sys::jint,
) -> jni::sys::jint {
    unsafe {
        JAVA_VM = env.get_java_vm().ok();
    }

    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(err) => {
            let _ = env.throw(err.to_string());
            return -1;
        }
    };

    let config = ServerConfig {
        port_start: port_start as u16,
        port_end: port_end as u16,
    };

    let result = runtime.block_on(async { start_server_once(config).await });

    match result {
        Ok(handle) => handle.port as jni::sys::jint,
        Err(err) => {
            let _ = env.throw(err.to_string());
            -1
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_server_RustServer_nativeStop(
    mut env: JNIEnv,
    _class: JClass,
) {
    let mut guard = SERVER_CELL.lock();
    if let Some(handle) = guard.as_ref() {
        let handle = handle.clone();
        let _ = tokio::runtime::Handle::try_current().map(|h| h.spawn(async move { handle.stop().await }));
    }

    if guard.take().is_some() {
            let _ = env.exception_occurred();
        }
}

async fn start_server_once(config: ServerConfig) -> Result<RustServerHandle, ServerError> {
    if (*SERVER_CELL.lock()).is_some() {
        return Err(ServerError::AlreadyStarted);
    }

    if config.port_start == 0 || config.port_end < config.port_start {
        return Err(ServerError::InvalidPort(format!(
            "{}..{}",
            config.port_start, config.port_end
        )));
    }

    let listener = find_available_port(config.port_start, config.port_end)
        .await
        .ok_or_else(|| ServerError::StartFailed("no available port".to_string()))?;

    let port = listener.local_addr().map(|a| a.port()).unwrap_or(config.port_start);
    let shutdown = Arc::new(AtomicBool::new(false));
    let cloned_shutdown = shutdown.clone();

    let server = axum::serve(
        listener,
        axum::Router::new()
            .route("/device", axum::routing::any(device_axum))
            .route("/tvbus", axum::routing::any(tvbus_axum))
            .route("/action", axum::routing::any(action_axum))
            .route("/media", axum::routing::any(media_axum))
            .route("/cache", axum::routing::any(cache_axum))
            .route("/parse", axum::routing::any(parse_axum))
            .route("/proxy", axum::routing::any(proxy_axum))
            .route("/newFolder", axum::routing::any(new_folder_axum))
            .route("/delFile", axum::routing::any(del_file_axum))
            .route("/delFolder", axum::routing::any(del_folder_axum))
            .route("/file/{*path}", axum::routing::any(file_axum))
            .route("/upload", axum::routing::any(upload_axum))
            .fallback(fallback_axum)
    )
    .with_graceful_shutdown(async move {
        loop {
            if cloned_shutdown.load(Ordering::SeqCst) {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    });

    let handle = tokio::spawn(async move {
        if let Err(err) = server.await {
            eprintln!("rust http server error: {}", err);
        }
    });

    let server_handle = RustServerHandle {
        port,
        handle: Arc::new(handle),
        shutdown,
    };

    *SERVER_CELL.lock() = Some(server_handle.clone());

    Ok(server_handle)
}

async fn find_available_port(start: u16, end: u16) -> Option<TcpListener> {
    for port in start..=end {
        match TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))).await {
            Ok(listener) => return Some(listener),
            Err(_) => continue,
        }
    }
    None
}

async fn request_headers(headers: axum::http::HeaderMap) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for (key, value) in headers.iter() {
        out.insert(key.to_string(), value.to_str().unwrap_or_default().to_string());
    }
    out
}

fn default_handler(_request: RequestView<'_>) -> ResponseView {
    ResponseView::not_found()
}

async fn handle_request(
    method: axum::http::Method,
    uri: axum::http::Uri,
    query: HashMap<String, String>,
    body: Vec<u8>,
    headers: HashMap<String, String>,
) -> impl axum::response::IntoResponse {
    let request = RequestView {
        method: method.as_str(),
        path: uri.path(),
        query: &query.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&"),
        body: body.as_ref(),
        headers: &headers,
    };

    let response = call_java_or_fallback(request);
    let builder = Response::builder()
        .status(axum::http::StatusCode::from_u16(response.status).unwrap_or(axum::http::StatusCode::OK))
        .header("Content-Type", response.content_type);
    builder.body(axum::body::Body::from(response.body)).unwrap_or_else(|_| Response::new(axum::body::Body::default()))
}

async fn device_axum(
    _method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
) -> impl axum::response::IntoResponse {
    handle_request(_method, axum::http::Uri::from_static("/device"), query.0, Vec::new(), request_headers(headers).await).await
}

async fn tvbus_axum(
    _method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
) -> impl axum::response::IntoResponse {
    handle_request(_method, axum::http::Uri::from_static("/tvbus"), query.0, Vec::new(), request_headers(headers).await).await
}

async fn action_axum(
    _method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
    body: axum::body::Bytes,
) -> impl axum::response::IntoResponse {
    handle_request(_method, axum::http::Uri::from_static("/action"), query.0, body.to_vec(), HashMap::new()).await
}

async fn media_axum(
    _method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    handle_request(_method, axum::http::Uri::from_static("/media"), query.0, Vec::new(), HashMap::new()).await
}

async fn cache_axum(
    _method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    handle_request(_method, axum::http::Uri::from_static("/cache"), query.0, Vec::new(), HashMap::new()).await
}

async fn parse_axum(
    _method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    let result = call_java_handler("/parse", &query.0.iter().map(|(k,v)| format!("{k}={v}")).collect::<Vec<_>>().join("&"), &[]);
    let body = result.as_ref().map(|r| r.body.clone()).unwrap_or_default();
    let content_type = result.as_ref().map(|r| r.content_type.clone()).unwrap_or_default();
    let mut response = Response::new(axum::body::Body::from(body));
    *response.status_mut() = axum::http::StatusCode::OK;
    response.headers_mut().insert("Content-Type", if content_type.is_empty() { axum::http::HeaderValue::from_static("text/html") } else { axum::http::HeaderValue::from_str(&content_type).unwrap_or_else(|_| axum::http::HeaderValue::from_static("text/html")) });
    response
}

#[derive(Debug, Deserialize)]
struct ProxyResponse {
    status: u16,
    mime: String,
    headers: HashMap<String, String>,
    body: String,
}

async fn proxy_axum(
    _method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> impl axum::response::IntoResponse {
    let mut merged = query.0;
    let mut headers_map = HashMap::new();
    for (key, value) in headers.iter() {
        headers_map.insert(key.to_string(), value.to_str().unwrap_or_default().to_string());
    }
    let headers_json = match serde_json::to_string(&headers_map) {
        Ok(json) => json,
        Err(_) => "{}".to_string(),
    };
    merged.insert("_headers".to_string(), headers_json);
    let query_string = merged.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&");

    let result = call_java_handler("/proxy", &query_string, &body);
    let text = result.as_ref().map(|r| String::from_utf8_lossy(&r.body).to_string()).unwrap_or_default();

    match serde_json::from_str::<ProxyResponse>(&text) {
        Ok(parsed) => {
            let body_bytes = general_purpose::STANDARD.decode(&parsed.body).unwrap_or_default();
            let mut builder = Response::builder()
                .status(parsed.status)
                .header("Content-Type", parsed.mime);
            for (key, value) in parsed.headers {
                builder = builder.header(key, value);
            }
            builder.body(axum::body::Body::from(body_bytes)).unwrap_or_else(|_| Response::new(axum::body::Body::from(String::new())))
        }
        Err(_) => {
            let mut response = Response::new(axum::body::Body::from(text));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            response.headers_mut().insert("Content-Type", HeaderValue::from_static("text/plain"));
            response
        }
    }
}

async fn new_folder_axum(
    method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    handle_request(method, axum::http::Uri::from_static("/newFolder"), query.0, Vec::new(), HashMap::new()).await
}

async fn del_file_axum(
    method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    handle_request(method, axum::http::Uri::from_static("/delFile"), query.0, Vec::new(), HashMap::new()).await
}

async fn del_folder_axum(
    method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    handle_request(method, axum::http::Uri::from_static("/delFolder"), query.0, Vec::new(), HashMap::new()).await
}

async fn file_axum(
    _method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
    axum::extract::Path(path): axum::extract::Path<String>,
    headers: axum::http::HeaderMap,
) -> impl axum::response::IntoResponse {
    let range = headers.get("range").and_then(|v| v.to_str().ok()).unwrap_or_default().to_string();
    let file_path = path.strip_prefix("/").unwrap_or(&path).to_string();
    let query_string = if range.is_empty() {
        query.0.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&")
    } else {
        let mut merged = query.0.clone();
        merged.insert("range".to_string(), range);
        merged.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&")
    };
    let query_string = if file_path.is_empty() {
        query_string
    } else {
        let mut merged = query.0.clone();
        merged.insert("path".to_string(), format!("/file/{}", file_path));
        merged.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&")
    };
    let result = call_java_handler("/file", &query_string, &[]);
    let text = result.as_ref().map(|r| String::from_utf8_lossy(&r.body).to_string()).unwrap_or_default();

    match serde_json::from_str::<ProxyResponse>(&text) {
        Ok(parsed) => {
            let body_bytes = general_purpose::STANDARD.decode(&parsed.body).unwrap_or_default();
            let mut builder = Response::builder()
                .status(parsed.status)
                .header("Content-Type", parsed.mime);
            for (key, value) in parsed.headers {
                builder = builder.header(key, value);
            }
            builder.body(axum::body::Body::from(body_bytes)).unwrap_or_else(|_| Response::new(axum::body::Body::from(String::new())))
        }
        Err(_) => {
            let mut response = Response::new(axum::body::Body::from(text));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            response.headers_mut().insert("Content-Type", HeaderValue::from_static("text/plain"));
            response
        }
    }
}

async fn upload_axum(
    _method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> impl axum::response::IntoResponse {
    let content_type = headers.get("content-type").and_then(|v| v.to_str().ok()).unwrap_or_default().to_string();
    let items = collect_upload_items(body.as_ref(), &content_type);
    let body = match serde_json::to_string(&items) {
        Ok(json) => json.into_bytes(),
        Err(_) => "[]".as_bytes().to_vec(),
    };

    let query_string = query.0.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&");
    let result = call_java_handler("/upload", &query_string, &body);
    let text = result.as_ref().map(|r| String::from_utf8_lossy(&r.body).to_string()).unwrap_or_default();
    let mut response = Response::new(axum::body::Body::from(text));
    *response.status_mut() = axum::http::StatusCode::OK;
    response.headers_mut().insert("Content-Type", HeaderValue::from_static("text/plain"));
    response
}

#[derive(Debug, Serialize, Deserialize)]
struct UploadItemJson {
    filename: String,
    temp_path: String,
    is_zip: bool,
}

fn collect_upload_items(body: &[u8], content_type: &str) -> Vec<UploadItemJson> {
    let mut items = Vec::new();
    let boundary = match parse_multipart_boundary(content_type) {
        Some(b) => b,
        None => return items,
    };
    for (filename, temp_path, is_zip) in parse_multipart_fields(body, &boundary) {
        items.push(UploadItemJson {
            filename,
            temp_path: temp_path.to_string_lossy().to_string(),
            is_zip,
        });
    }
    items
}

fn parse_multipart_boundary(content_type: &str) -> Option<String> {
    for part in content_type.split(';') {
        let part = part.trim();
        if let Some(rest) = part.strip_prefix("boundary=") {
            return Some(rest.trim_matches('"').to_string());
        }
    }
    None
}

fn parse_multipart_fields(body: &[u8], boundary: &str) -> Vec<(String, std::path::PathBuf, bool)> {
    let mut items = Vec::new();
    let delimiter = format!("--{}", boundary);
    let end_delimiter = format!("{}--", delimiter);
    let data = std::str::from_utf8(body).unwrap_or_default();
    let mut parts = data.split(&delimiter);
    while let Some(part) = parts.next() {
        if part.trim_start().starts_with("--") {
            break;
        }
        let mut header_lines = part.lines();
        let mut disposition: Option<String> = None;
        while let Some(line) = header_lines.next() {
            if line.is_empty() {
                break;
            }
            if let Some(rest) = line.strip_prefix("Content-Disposition:") {
                disposition = Some(rest.trim().to_string());
            }
        }
        let disposition = match disposition {
            Some(d) => d,
            None => continue,
        };
        let filename = disposition
            .split(';')
            .find_map(|s| s.trim().strip_prefix("filename="))
            .map(|s| s.trim_matches('"').to_string())
            .unwrap_or_default();
        let name = disposition
            .split(';')
            .find_map(|s| s.trim().strip_prefix("name="))
            .map(|s| s.trim_matches('"').to_string())
            .unwrap_or_default();
        if name.is_empty() || filename.is_empty() {
            continue;
        }
        let body_start = part.find("

").unwrap_or(part.len());
        let content = part[body_start..].trim_end_matches("
");
        if content.starts_with(&end_delimiter) {
            continue;
        }
        let temp_path = std::env::temp_dir().join(format!("upload_{}_{}", name, filename));
        if std::fs::write(&temp_path, content).is_err() {
            continue;
        }
        let is_zip = filename.to_lowercase().ends_with(".zip");
        items.push((filename, temp_path, is_zip));
    }
    items
}

async fn fallback_axum(
    method: axum::http::Method,
    uri: axum::http::Uri,
    query: axum::extract::Query<HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    handle_request(method, uri, query.0, Vec::new(), HashMap::new()).await
}

fn call_java_or_fallback(request: RequestView<'_>) -> ResponseView {
    match call_java_handler(request.path, &request.query, &request.body) {
        Some(response) => response,
        None => default_handler(request),
    }
}

fn call_java_handler(path: &str, query: &str, body: &[u8]) -> Option<ResponseView> {
    let vm = java_vm_ref()?;
    {
        let mut env = vm.attach_current_thread_as_daemon().ok()?;

        let path = env.new_string(path).ok()?;
        let query = env.new_string(query).ok()?;
        let body = env.byte_array_from_slice(body).ok()?;

        let result = env
            .call_static_method(
                "com/fongmi/android/tv/server/RustServerCallback",
                "onHandle",
                "(Ljava/lang/String;Ljava/lang/String;[B)Ljava/lang/String;",
                &[JValueGen::from(&path), JValueGen::from(&query), JValueGen::from(&body)],
            )
            .ok()?;

        let value: JString = result.l().ok()?.into();
        let text: String = env.get_string(&value).ok()?.into();
        drop(env);

        Some(ResponseView::ok_text(text))
    }
}

