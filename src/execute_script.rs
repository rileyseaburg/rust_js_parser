use ssr_rs::v8;
use super::JsHttpRequestProcessor;

impl<'s, 'i> JsHttpRequestProcessor<'s, 'i>
where
    's: 'i,
{
    pub fn execute_script(&mut self, script: v8::Local<'s, v8::String>) {
        let scope = &mut v8::HandleScope::new(&mut self.context_scope);
        let try_catch = &mut v8::TryCatch::new(scope);

        let script =
            v8::Script::compile(try_catch, script, None).expect("failed to compile script");

        if script.run(try_catch).is_none() {
            let exception = try_catch.exception().unwrap();
            let exception_string = exception.to_rust_string_lossy(try_catch);
            panic!("{exception_string}");
        }
    }
}
