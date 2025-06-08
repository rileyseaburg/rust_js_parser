use crate::{react_compiler, ssr, StringHttpRequest};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web::web::Data;

/// Basic route handler for JavaScript processing
pub async fn handle_js_request(req: HttpRequest) -> Result<HttpResponse> {
    // Extract request information
    let path = req.path();
    let host = req
        .headers()
        .get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost");
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    let referer = req
        .headers()
        .get("referer")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    // Create StringHttpRequest
    let _string_request = StringHttpRequest::new(path, host, user_agent, referer);

    // For now, return a simple response indicating the request was processed
    // In the future, this will process JavaScript and return rendered content
    Ok(HttpResponse::Ok().content_type("text/html").body(format!(
        "<html><body><h1>JavaScript Processing</h1><p>Path: {}</p><p>Host: {}</p></body></html>",
        path, host
    )))
}

/// Route handler for React component rendering
pub async fn handle_react_render() -> Result<HttpResponse> {
    let react_compiler = react_compiler::ReactCompiler::new();

    // Step 1: Compile the React component file (this is just JSX-to-JS via SWC)
    let compiled_component = react_compiler
        .compile_react_file("./src/examples/react-component.tsx")
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to compile React component file: {}",
                e
            ))
        })?;

    // Step 2: Create a proper CommonJS environment for SSR
    let wrapped_ssr_js = format!(r#"
        // Mock require function for CommonJS modules
        globalThis.require = function(moduleName) {{
            if (moduleName === 'react') {{
                return {{
                    createElement: function(type, props, ...children) {{
                        return {{ type, props: props || {{}}, children }};
                    }},
                    Fragment: 'Fragment'
                }};
            }}
            if (moduleName === 'react-dom/server') {{
                return {{
                    renderToString: function(element) {{
                        if (!element) return '';
                        if (typeof element === 'string') return element;
                        if (typeof element === 'number') return String(element);
                        
                        const {{ type, props, children }} = element;
                        if (typeof type === 'string') {{
                            let html = `<${{type}}`;
                            if (props) {{
                                for (const [key, value] of Object.entries(props)) {{
                                    if (key !== 'children' && value != null) {{
                                        html += ` ${{key}}="${{String(value)}}"`;
                                    }}
                                }}
                            }}
                            html += '>';
                            
                            if (children && children.length > 0) {{
                                for (const child of children) {{
                                    html += this.renderToString(child);
                                }}
                            }}
                            
                            html += `</${{type}}>`;
                            return html;
                        }}
                        return '';
                    }}
                }};
            }}
            return {{}};
        }};

        // Mock module and exports
        globalThis.module = {{ exports: {{}} }};
        globalThis.exports = globalThis.module.exports;

        {compiled}

        // Entry point function - make it global
        globalThis.entrypoint = function() {{
            const Component = module.exports.default;
            if (!Component) {{ throw new Error('No default export found'); }}
            const React = require('react');
            const ReactDOMServer = require('react-dom/server');
            return ReactDOMServer.renderToString(
                React.createElement(Component, {{ name: 'World', age: 25 }})
            );
        }};
        "#, compiled = compiled_component);

    println!("Wrapped SSR JS code:\n{}", wrapped_ssr_js);

    // Step 3: Initialize SSR runtime with this wrapped JS code
    ssr_rs::Ssr::create_platform();

    println!("Initializing SSR with wrapped JS code...");

    let mut ssr_instance = ssr_rs::Ssr::from(wrapped_ssr_js, "entrypoint")
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("SSR init error: {}", e)))
        .expect("Failed to initialize SSR");
        

    // Step 4: Render to string using SSR
    let rendered_html = ssr_instance
        .render_to_string(None)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("SSR render error: {}", e)))?;

    println!("React component rendered successfully");

    // Step 5: Return full HTML
    let full_html = format!(
        r###"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>React SSR</title>
        </head>
        <body>
            <div id="app">{}</div>
        </body>
        </html>
        "###,
        rendered_html
    );

    Ok(HttpResponse::Ok().content_type("text/html").body(full_html))
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
pub async fn start_server() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");

    HttpServer::new(|| create_app())
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
