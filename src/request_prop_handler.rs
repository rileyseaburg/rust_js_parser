use ssr_rs::v8;
use super::JsHttpRequestProcessor;
use crate::ssr::http_request::SimpleHttpRequest;

impl<'s, 'i> JsHttpRequestProcessor<'s, 'i>
where
    's: 'i,
{
    /// This handles the properties of `HttpRequest`
    #[allow(clippy::needless_pass_by_value)] // this function should follow the callback type
    pub fn request_prop_handler(
        scope: &mut v8::HandleScope,
        key: v8::Local<v8::Name>,
        args: v8::PropertyCallbackArguments,
        mut rv: v8::ReturnValue,
    ) {
        let this = args.this();
        let external = Self::unwrap_request(scope, this);

        assert!(
            !external.is_null(),
            "the pointer to Box<dyn HttpRequest> should not be null"
        );

        let request = unsafe { &mut *external };

        let key = key.to_rust_string_lossy(scope);

        let value = match &*key {
            "path" => request.path(),
            "userAgent" => request.user_agent(),
            "referrer" => request.referrer(),
            "host" => request.host(),
            _ => {
                return;
            }
        };

        rv.set(v8::String::new(scope, value).unwrap().into());
    }
}
