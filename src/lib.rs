use ssr_rs::v8;
use std::collections::HashMap;

pub mod actix_integration;
pub mod create_script_origin;
pub mod examples;
pub mod execute_script;
pub mod js_parser;
pub mod new;
pub mod print_output;
pub mod process;
pub mod react_compiler;
pub mod request_prop_handler;
pub mod simple_tests;
pub mod ssr;
pub mod unwrap_request;
pub mod wrap_map;
pub mod wrap_request;
pub mod send_wrapper;

pub use create_script_origin::*;
pub use execute_script::*;
pub use new::*;
pub use print_output::*;
pub use process::*;
pub use request_prop_handler::*;
pub use unwrap_request::*;
pub use wrap_map::*;
pub use wrap_request::*;


#[cfg(test)]
mod test;

#[allow(clippy::needless_pass_by_value)] // this function should follow the callback type
pub fn log_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    let message = args.get(0).to_rust_string_lossy(scope);

    println!("Logged: {message}");
}

#[allow(clippy::needless_pass_by_value)] // this function should follow the callback type
pub fn require_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let module_name = args.get(0).to_rust_string_lossy(scope);
    
    // Basic module resolution for common modules
    let module_exports = match module_name.as_str() {
        "react" => {
            // Simple React mock
            let react_obj = v8::Object::new(scope);
            let create_element_fn = v8::Function::new(
                scope,
                |scope: &mut v8::HandleScope,
                 args: v8::FunctionCallbackArguments,
                 mut retval: v8::ReturnValue| {
                    // Simple createElement implementation
                    let obj = v8::Object::new(scope);
                    let type_key = v8::String::new(scope, "type").unwrap();
                    let props_key = v8::String::new(scope, "props").unwrap();
                    
                    if args.length() > 0 {
                        obj.set(scope, type_key.into(), args.get(0));
                    }
                    if args.length() > 1 {
                        obj.set(scope, props_key.into(), args.get(1));
                    }
                    
                    retval.set(obj.into());
                },
            ).unwrap();
            
            let create_element_key = v8::String::new(scope, "createElement").unwrap();
            react_obj.set(scope, create_element_key.into(), create_element_fn.into());
            react_obj
        },
        _ => {
            // Return empty object for unknown modules
            v8::Object::new(scope)
        }
    };
    
    retval.set(module_exports.into());
}

use send_wrapper::SendWrapper;

/// An http request processor that is scriptable using JavaScript.
pub struct JsHttpRequestProcessor<'s, 'i> {
    pub context: SendWrapper<v8::Local<'s, v8::Context>>,
    pub context_scope: SendWrapper<v8::ContextScope<'i, v8::HandleScope<'s>>>,
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
