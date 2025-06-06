use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use crate::StringHttpRequest;

/// Basic route handler for JavaScript processing
pub async fn handle_js_request(req: HttpRequest) -> Result<HttpResponse> {
    // Extract request information
    let path = req.path();
    let host = req.headers()
        .get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost");
    let user_agent = req.headers()
        .get("user-agent")  
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    let referer = req.headers()
        .get("referer")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    // Create StringHttpRequest
    let _string_request = StringHttpRequest::new(path, host, user_agent, referer);
    
    // For now, return a simple response indicating the request was processed
    // In the future, this will process JavaScript and return rendered content
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(format!(
            "<html><body><h1>JavaScript Processing</h1><p>Path: {}</p><p>Host: {}</p></body></html>",
            path, host
        )))
}

/// Route handler for React component rendering
pub async fn handle_react_render() -> Result<HttpResponse> {
    // This will eventually render React components using swc_ecma_react_compiler
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("<html><body><h1>React Component Rendered</h1></body></html>"))
}

/// Create and configure the Actix-Web application
pub fn create_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .route("/", web::get().to(handle_js_request))
        .route("/js/*", web::get().to(handle_js_request))
        .route("/react", web::get().to(handle_react_render))
}

/// Start the Actix-Web server
#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");
    
    HttpServer::new(|| create_app())
        .bind("127.0.0.1:8080")?
        .run()
        .await
}