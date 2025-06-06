use ssr_rs::v8;
use super::JsHttpRequestProcessor;
use crate::ssr::http_request::SimpleHttpRequest;

impl<'s, 'i> JsHttpRequestProcessor<'s, 'i>
where
    's: 'i,
{
    /// Utility function that extracts the http request object from a wrapper object.
    pub fn unwrap_request(
        scope: &mut v8::HandleScope,
        request: v8::Local<v8::Object>,
    ) -> *mut Box<dyn SimpleHttpRequest> {
        let external = request
            .get_internal_field(scope, 0)
            .unwrap()
            .cast::<v8::External>();
        external.value() as *mut Box<dyn SimpleHttpRequest>
    }
}
