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
