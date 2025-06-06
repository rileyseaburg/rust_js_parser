/// A trait representing HTTP request information needed for server-side rendering
pub trait SimpleHttpRequest {
    /// Returns the request path
    fn path(&self) -> &str;
    
    /// Returns the user agent header
    fn user_agent(&self) -> &str;
    
    /// Returns the referrer header  
    fn referrer(&self) -> &str;
    
    /// Returns the host header
    fn host(&self) -> &str;
}