use base64::{engine::general_purpose, Engine as _};
use http::Response;
use axum_multipart::Multipart;
use jni::objects::{JClass, JString, JavaVM};
use jni::sys::jstring;
use jni::{AttachArgs, JNIEnv};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::sync::OnceCell;
use tokio::task::JoinHandle;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

static SERVER_CELL: OnceCell<RustServerHandle> = OnceCell::new();
static mut JAVA_VM: Option<JavaVM> = None;

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
) -> jstring {
    unsafe {
        JAVA_VM = env.get_java_vm().ok();
    }

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(server_error_to_string);

    let runtime = match runtime {
        Ok(rt) => rt,
        Err(err) => return throw_and_empty_string(&mut env, err),
    };

    let config = ServerConfig {
        port_start: port_start as u16,
        port_end: port_end as u16,
    };

    let result = runtime.block_on(async { start_server_once(config).await });

    match result {
        Ok(handle) => match env.new_string(format!("{}", handle.port)) {
            Ok(s) => s.into_raw(),
            Err(err) => throw_and_empty_string(&mut env, ServerError::StartFailed(err.to_string())),
        },
        Err(err) => throw_and_empty_string(&mut env, err),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_server_RustServer_nativeStop(
    env: JNIEnv,
    _class: JClass,
) {
    if let Some(handle) = SERVER_CELL.get() {
        let handle = handle.clone();
        let _ = tokio::runtime::Handle::try_current().map(|h| h.spawn(async move { handle.stop().await }));
    }

    if let Err(err) = SERVER_CELL.try_reset() {
        let _ = env.exception_occurred();
    }
}

async fn start_server_once(config: ServerConfig) -> Result<RustServerHandle, ServerError> {
    if SERVER_CELL.get().is_some() {
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
            .route("/file", axum::routing::any(file_axum))
            .route("/upload", axum::routing::any(upload_axum))
            .fallback(fallback_axum)
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CorsLayer::permissive()),
            )
            .into_make_service_with_connect_info::<SocketAddr>(),
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
            tracing::warn!(%err, "rust http server error");
        }
    });

    let server_handle = RustServerHandle {
        port,
        handle,
        shutdown,
    };

    SERVER_CELL
        .set(server_handle.clone())
        .expect("server cell set failed");

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

fn default_handler(request: RequestView<'_>) -> ResponseView {
    ResponseView::ok_text(format!("rust-fallback {} {}", request.method, request.path))
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
        query: query.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&"),
        body,
        headers,
    };

    let response = call_java_or_fallback(request);
    axum::response::IntoResponse::into_response((
        axum::http::StatusCode::from_u16(response.status).unwrap_or(axum::http::StatusCode::OK),
        response.content_type,
        response.body,
    ))
}

async fn device_axum(
    method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
) -> impl axum::response::IntoResponse {
    handle_request(method, axum::http::Uri::from_static("/device"), query.0, Vec::new(), request_headers(headers).await).await
}

async fn tvbus_axum(
    method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
) -> impl axum::response::IntoResponse {
    handle_request(method, axum::http::Uri::from_static("/tvbus"), query.0, Vec::new(), request_headers(headers).await).await
}

async fn action_axum(
    method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
    body: axum::extract::DefaultBodyLimit(axum::body::Bytes),
) -> impl axum::response::IntoResponse {
    handle_request(method, axum::http::Uri::from_static("/action"), query.0, body.0.to_vec(), HashMap::new()).await
}

async fn media_axum(
    method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    handle_request(method, axum::http::Uri::from_static("/media"), query.0, Vec::new(), HashMap::new()).await
}

async fn cache_axum(
    method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    handle_request(method, axum::http::Uri::from_static("/cache"), query.0, Vec::new(), HashMap::new()).await
}

async fn parse_axum(
    method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    let result = call_java_handler("/parse", &query.0.iter().map(|(k,v)| format!("{k}={v}")).collect::<Vec<_>>().join("&"), &[]);
    let body = result.as_ref().map(|r| r.body.clone()).unwrap_or_default();
    let content_type = result.as_ref().map(|r| r.content_type.clone()).unwrap_or_default();
    (
        axum::http::StatusCode::OK,
        if content_type.is_empty() { axum::http::HeaderValue::from_static("text/html") } else { axum::http::HeaderValue::from_str(&content_type).unwrap_or_else(|_| axum::http::HeaderValue::from_static("text/html")) },
        body,
    )
}

#[derive(Debug, Deserialize)]
struct ProxyResponse {
    status: u16,
    mime: String,
    headers: HashMap<String, String>,
    body: String,
}

async fn proxy_axum(
    method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    body: axum::extract::DefaultBodyLimit(axum::body::Bytes),
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

    let result = call_java_handler("/proxy", &query_string, &body.0);
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
            *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
            response.headers_mut().insert("Content-Type", http::HeaderValue::from_static("text/plain"));
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
    method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
) -> impl axum::response::IntoResponse {
    let range = headers.get("range").and_then(|v| v.to_str().ok()).unwrap_or_default().to_string();
    let query_string = if range.is_empty() {
        query.0.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&")
    } else {
        let mut merged = query.0.clone();
        merged.insert("range".to_string(), range);
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
            *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
            response.headers_mut().insert("Content-Type", http::HeaderValue::from_static("text/plain"));
            response
        }
    }
}

async fn upload_axum(
    method: axum::http::Method,
    query: axum::extract::Query<HashMap<String, String>>,
    mut multipart: Multipart,
) -> impl axum::response::IntoResponse {
    let items = collect_upload_items(&mut multipart).await;
    let body = match serde_json::to_string(&items) {
        Ok(json) => json.into_bytes(),
        Err(_) => "[]".as_bytes().to_vec(),
    };

    let query_string = query.0.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&");
    let result = call_java_handler("/upload", &query_string, &body);
    let text = result.as_ref().map(|r| String::from_utf8_lossy(&r.body).to_string()).unwrap_or_default();
    (
        axum::http::StatusCode::OK,
        axum::http::HeaderValue::from_static("text/plain"),
        axum::body::Body::from(text),
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct UploadItemJson {
    filename: String,
    temp_path: String,
    is_zip: bool,
}

async fn collect_upload_items(multipart: &mut Multipart) -> Vec<UploadItemJson> {
    let mut items = Vec::new();
    while let Ok(Some(mut field)) = multipart.next_field().await {
        let filename = field.file_name().unwrap_or_default().to_string();
        let name = field.name().unwrap_or_default().to_string();
        if name.is_empty() || filename.is_empty() {
            continue;
        }

        let temp_path = std::env::temp_dir().join(format!("upload_{}_{}", name, filename));
        let mut file = match tokio::fs::File::create(&temp_path).await {
            Ok(f) => f,
            Err(_) => continue,
        };

        let mut is_zip = filename.to_lowercase().ends_with(".zip");
        while let Ok(Some(chunk)) = field.chunk().await {
            if chunk.starts_with(b"%PDF-") || chunk.len() > 8 {
                is_zip = false;
            }
            if let Err(_) = tokio::io::copy(&mut std::io::Cursor::new(chunk), &mut file).await {
                break;
            }
        }
        drop(file);

        items.push(UploadItemJson {
            filename,
            temp_path: temp_path.to_string_lossy().to_string(),
            is_zip,
        });
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
    unsafe {
        let vm = JAVA_VM.as_ref()?;
        let mut env = vm.attach_current_thread_as_daemon(AttachArgs::default()).ok()?;

        let path = env.new_string(path).ok()?;
        let query = env.new_string(query).ok()?;
        let body = env.byte_array_from_slice(body).ok()?;

        let result = env
            .call_static_method(
                "com/fongmi/android/tv/server/RustServerCallback",
                "onHandle",
                "(Ljava/lang/String;Ljava/lang/String;[B)Ljava/lang/String;",
                &[path.into(), query.into(), body.into()],
            )
            .ok()?;

        let value: JString = result.l().ok()?;
        let text: String = env.get_string(&value).ok()?.into();
        drop(env);

        Some(ResponseView::ok_text(text))
    }
}

fn server_error_to_string(err: impl std::fmt::Display) -> ServerError {
    ServerError::StartFailed(err.to_string())
}

fn throw_and_empty_string<'local>(env: &mut JNIEnv<'local>, err: impl std::fmt::Display) -> jstring {
    let description = err.to_string();
    let _ = env.throw(description.clone());
    env.new_string("").unwrap().into_raw()
}
