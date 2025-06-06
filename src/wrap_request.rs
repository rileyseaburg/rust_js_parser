use ssr_rs::v8;
use super::JsHttpRequestProcessor;
use crate::ssr::http_request::SimpleHttpRequest;
use std::ffi::c_void;

impl<'s, 'i> JsHttpRequestProcessor<'s, 'i>
where
    's: 'i,
{
    /// Utility function that wraps a http request object in a JavaScript object.
    pub fn wrap_request(&mut self, request: Box<dyn SimpleHttpRequest>) -> v8::Local<'s, v8::Object> {
        // TODO: fix memory leak

        // Double-box to get C-sized reference of Box<dyn SimpleHttpRequest>
        let request = Box::new(request);

        // Local scope for temporary handles.
        let scope = &mut self.context_scope;

        let request_template = v8::Local::new(scope, &self.request_template);
        let result = request_template.new_instance(scope).unwrap();

        let external = v8::External::new(
            scope,
            Box::leak(request) as *mut Box<dyn SimpleHttpRequest> as *mut c_void,
        );

        result.set_internal_field(0, external.into());

        let name = v8::String::new(scope, "path").unwrap().into();
        result.set_accessor(scope, name, Self::request_prop_handler);
        let name = v8::String::new(scope, "userAgent").unwrap().into();
        result.set_accessor(scope, name, Self::request_prop_handler);
        let name = v8::String::new(scope, "referrer").unwrap().into();
        result.set_accessor(scope, name, Self::request_prop_handler);
        let name = v8::String::new(scope, "host").unwrap().into();
        result.set_accessor(scope, name, Self::request_prop_handler);

        result
    }
}
