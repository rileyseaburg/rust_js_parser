use ssr_rs::v8;
use super::JsHttpRequestProcessor;
use crate::ssr::http_request::SimpleHttpRequest;

impl<'s, 'i> JsHttpRequestProcessor<'s, 'i>
where
    's: 'i,
{
    /// Processes the given HTTP request.
    pub fn process<R>(&mut self, request: R)
    where
        R: SimpleHttpRequest + 'static,
    {
        let request: Box<dyn SimpleHttpRequest> = Box::new(request);
        let request = self.wrap_request(request);

        let scope = &mut v8::HandleScope::new(&mut self.context_scope);
        let try_catch = &mut v8::TryCatch::new(scope);

        let process_fn = self.process_fn.as_mut().unwrap();
        let global = self.context.global(try_catch).into();

        if process_fn
            .call(try_catch, global, &[request.into()][..])
            .is_none()
        {
            let exception = try_catch.exception().unwrap();
            let exception_string = exception.to_rust_string_lossy(try_catch);

            panic!("{exception_string}");
        }
    }
}
