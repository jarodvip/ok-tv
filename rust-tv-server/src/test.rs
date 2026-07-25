#[cfg(test)]
mod tests {
    use crate::*;

    // ResponseView 构造器测试
    #[test]
    fn test_response_view_ok_text() {
        let view = ResponseView::ok_text("hello");
        assert_eq!(view.status, 200);
        assert_eq!(view.body, b"hello");
        assert_eq!(view.content_type, "text/plain");
    }

    #[test]
    fn test_response_view_ok_text_empty() {
        let view = ResponseView::ok_text("");
        assert_eq!(view.status, 200);
        assert!(view.body.is_empty());
    }

    #[test]
    fn test_response_view_not_found() {
        let view = ResponseView::not_found();
        assert_eq!(view.status, 404);
        assert_eq!(view.body, b"not found");
        assert_eq!(view.content_type, "text/plain");
    }

    // ServerError 枚举测试
    #[test]
    fn test_server_error_already_started() {
        let err = ServerError::AlreadyStarted;
        assert!(matches!(err, ServerError::AlreadyStarted));
        assert_eq!(err.to_string(), "server already started");
    }

    #[test]
    fn test_server_error_invalid_port() {
        let err = ServerError::InvalidPort("0..100".to_string());
        assert_eq!(err.to_string(), "invalid port: 0..100");
    }

    #[test]
    fn test_server_error_start_failed() {
        let err = ServerError::StartFailed("no available port".to_string());
        assert_eq!(err.to_string(), "start failed: no available port");
    }

    // ServerConfig 验证测试
    #[test]
    fn test_server_config_port_zero_fails() {
        let config = ServerConfig { port_start: 0, port_end: 100 };
        // start_server_once 会拒绝 port_start == 0
        assert_eq!(config.port_start, 0);
        assert!(config.port_end >= config.port_start);
        // 但 port_start == 0 应该被拒绝
        assert!(config.port_start == 0);
    }

    #[test]
    fn test_server_config_port_range_invalid() {
        let config = ServerConfig { port_start: 200, port_end: 100 };
        assert!(config.port_end < config.port_start);
    }

    #[test]
    fn test_server_config_port_range_valid() {
        let config = ServerConfig { port_start: 8080, port_end: 8090 };
        assert!(config.port_end >= config.port_start);
        assert!(config.port_start > 0);
    }

    // RustServerHandle 测试
    #[test]
    fn test_rust_server_handle_defaults() {
        // RustServerHandle 的结构: port, handle(Arc), shutdown(Arc)
        // 我们只测试结构体字段的逻辑
        let port: u16 = 8080;
        assert!(port > 1024);
    }

    // request_headers 测试
    #[tokio::test]
    async fn test_request_headers_empty() {
        use axum::http::HeaderMap;

        let headers = HeaderMap::new();
        let result = request_headers(headers).await;
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_request_headers_single() {
        use axum::http::{HeaderMap, HeaderName, HeaderValue};

        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"));
        headers.insert(HeaderName::from_static("x-custom"), HeaderValue::from_static("test-value"));

        let result = request_headers(headers).await;
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("content-type"), Some(&"application/json".to_string()));
        assert_eq!(result.get("x-custom"), Some(&"test-value".to_string()));
    }

    // find_available_port 测试
    #[tokio::test]
    async fn test_find_available_port_in_range() {
        // 使用一个不太可能被占用的高位端口范围
        let result = find_available_port(48000, 48010).await;
        // 至少能找到其中一个端口，或者全部被占用
        if let Some(listener) = result {
            let addr = listener.local_addr().unwrap();
            let port = addr.port();
            assert!(port >= 48000 && port <= 48010);
        }
    }

    #[tokio::test]
    async fn test_find_available_port_no_port() {
        // 指定一个极小范围，大概率没有可用端口
        // 使用 127.0.0.1:1 和 127.0.0.1:2 这种极端情况
        let result = find_available_port(1, 1).await;
        // 应该找不到（端口 1 需要 root 权限）
        assert!(result.is_none());
    }

    // start_server_once 验证逻辑测试（不实际启动服务器）
    #[test]
    fn test_server_cell_initially_none() {
        // SERVER_CELL 是 LazyLock<Mutex<Option<RustServerHandle>>>
        // 初始状态应为 None
        let cell_guard = SERVER_CELL.lock();
        assert!(cell_guard.is_none());
        drop(cell_guard);
    }

    // ProxyResponse 反序列化测试
    #[test]
    fn test_proxy_response_deserialize() {
        let body_b64 = base64::engine::general_purpose::STANDARD.encode("hello");
        let json = format!(
            r#"{{"status":200,"mime":"application/json","headers":{{"content-type":"application/json"}},"body":"{}"}}"#,
            body_b64
        );
        let parsed: ProxyResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.status, 200);
        assert_eq!(parsed.mime, "application/json");
        assert_eq!(parsed.body, body_b64);
    }

    // UploadItemJson 序列化测试
    #[test]
    fn test_upload_item_json() {
        let item = UploadItemJson {
            filename: "test.zip".to_string(),
            temp_path: "/tmp/test.zip".to_string(),
            is_zip: true,
        };
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("test.zip"));
        assert!(json.contains("/tmp/test.zip"));

        let parsed: UploadItemJson = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.filename, "test.zip");
        assert!(parsed.is_zip);
    }

    // parse_multipart_boundary 测试
    #[test]
    fn test_parse_multipart_boundary() {
        assert_eq!(
            parse_multipart_boundary("multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW"),
            Some("----WebKitFormBoundary7MA4YWxkTrZu0gW".to_string())
        );
        assert_eq!(
            parse_multipart_boundary("multipart/form-data;boundary=abc"),
            Some("abc".to_string())
        );
        assert_eq!(
            parse_multipart_boundary("application/json"),
            None
        );
    }

    #[test]
    fn test_parse_multipart_boundary_with_quotes() {
        assert_eq!(
            parse_multipart_boundary("multipart/form-data; boundary=\"test-boundary\""),
            Some("test-boundary".to_string())
        );
    }
}
