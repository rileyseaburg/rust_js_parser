#[cfg(feature = "ssr")]
use ssr_rs::v8;
use std::collections::HashMap;

pub mod actix_integration;
pub mod examples;
#[cfg(feature = "ssr")]
pub mod execute_script;
pub mod js_parser;
#[cfg(feature = "ssr")]
pub mod new;
#[cfg(feature = "ssr")]
pub mod print_output;
#[cfg(feature = "ssr")]
pub mod process;
pub mod react_compiler;
#[cfg(feature = "ssr")]
pub mod request_prop_handler;
pub mod simple_tests;
pub mod ssr;
#[cfg(feature = "ssr")]
pub mod unwrap_request;
#[cfg(feature = "ssr")]
pub mod wrap_map;
#[cfg(feature = "ssr")]
pub mod wrap_request;

#[cfg(feature = "ssr")]
pub use execute_script::*;
#[cfg(feature = "ssr")]
pub use new::*;
#[cfg(feature = "ssr")]
pub use print_output::*;
#[cfg(feature = "ssr")]
pub use process::*;
#[cfg(feature = "ssr")]
pub use request_prop_handler::*;
#[cfg(feature = "ssr")]
pub use unwrap_request::*;
#[cfg(feature = "ssr")]
pub use wrap_map::*;
#[cfg(feature = "ssr")]
pub use wrap_request::*;


#[cfg(test)]
mod test;

#[allow(clippy::needless_pass_by_value)] // this function should follow the callback type
#[cfg(feature = "ssr")]
pub fn log_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    let message = args.get(0).to_rust_string_lossy(scope);

    println!("Logged: {message}");
}

/// An http request processor that is scriptable using JavaScript.
#[cfg(feature = "ssr")]
pub struct JsHttpRequestProcessor<'s, 'i> {
    pub context: v8::Local<'s, v8::Context>,
    pub context_scope: v8::ContextScope<'i, v8::HandleScope<'s>>,
    pub process_fn: Option<v8::Local<'s, v8::Function>>,
    pub request_template: v8::Global<v8::ObjectTemplate>,
    pub _map_template: Option<v8::Global<v8::ObjectTemplate>>,
}

#[derive(Debug, Clone)]
pub struct StringHttpRequest {
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl StringHttpRequest {
    pub fn new(path: &str, host: &str, user_agent: &str, referer: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("host".to_string(), host.to_string());
        headers.insert("user-agent".to_string(), user_agent.to_string());
        headers.insert("referer".to_string(), referer.to_string());

        Self {
            path: path.to_string(),
            headers,
            body: "".to_string(),
        }
    }
}

impl crate::ssr::http_request::SimpleHttpRequest for StringHttpRequest {
    fn path(&self) -> &str {
        &self.path
    }
    
    fn user_agent(&self) -> &str {
        self.headers.get("user-agent").map(|s| s.as_str()).unwrap_or("")
    }
    
    fn referrer(&self) -> &str {
        self.headers.get("referer").map(|s| s.as_str()).unwrap_or("")
    }
    
    fn host(&self) -> &str {
        self.headers.get("host").map(|s| s.as_str()).unwrap_or("")
    }
}
