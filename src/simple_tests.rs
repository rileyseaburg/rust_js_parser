#[cfg(test)]
mod test_simple {
    use crate::StringHttpRequest;
    use crate::ssr::http_request::SimpleHttpRequest;

    #[test]
    fn test_string_http_request_creation() {
        let request = StringHttpRequest::new("/test", "localhost", "test-agent", "http://test.com");
        
        assert_eq!(request.path(), "/test");
        assert_eq!(request.host(), "localhost");
        assert_eq!(request.user_agent(), "test-agent");
        assert_eq!(request.referrer(), "http://test.com");
    }
    
    #[test]
    fn test_string_http_request_trait() {
        let request = StringHttpRequest::new("/api/test", "example.com", "Mozilla/5.0", "https://example.com");
        
        // Test trait methods
        assert_eq!(request.path(), "/api/test");
        assert_eq!(request.host(), "example.com");
        assert_eq!(request.user_agent(), "Mozilla/5.0");
        assert_eq!(request.referrer(), "https://example.com");
    }
}