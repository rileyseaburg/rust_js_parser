use ssr_rs::v8;
use std::collections::HashMap;

pub mod execute_script;
pub mod new;
pub mod print_output;
pub mod process;
pub mod request_prop_handler;
pub mod unwrap_request;
pub mod wrap_map;
pub mod wrap_request;

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

/// An http request processor that is scriptable using JavaScript.
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
