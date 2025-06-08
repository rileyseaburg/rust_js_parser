use super::create_script_origin;
use super::JsHttpRequestProcessor;
use ssr_rs::v8;

impl<'s, 'i> JsHttpRequestProcessor<'s, 'i>
where
    's: 'i,
{
    pub fn execute_script(&mut self, script: v8::Local<'s, v8::String>) {
        let scope = &mut v8::HandleScope::new(&mut *self.context_scope);
        let try_catch = &mut v8::TryCatch::new(scope);

        let script =
            v8::Script::compile(try_catch, script, None).expect("failed to compile script");

        if script.run(try_catch).is_none() {
            let exception = try_catch.exception().unwrap();
            let exception_string = exception.to_rust_string_lossy(try_catch);
            panic!("{exception_string}");
        }
    }

    pub fn execute_module(&mut self, script: v8::Local<'s, v8::String>) {
        let scope = &mut v8::HandleScope::new(&mut *self.context_scope);
        let try_catch = &mut v8::TryCatch::new(scope);

        let origin = create_script_origin(try_catch, "file.js", true);
        let mut source = v8::script_compiler::Source::new(script, Some(&origin));

        let module = v8::script_compiler::compile_module(try_catch, &mut source)
            .expect("failed to compile module");

        if module.instantiate_module(try_catch, |_, _, _, _| None).is_none() {
            let exception = try_catch.exception().unwrap();
            let exception_string = exception.to_rust_string_lossy(try_catch);
            panic!("{exception_string}");
        };

        if module.evaluate(try_catch).is_none() {
            let exception = try_catch.exception().unwrap();
            let exception_string = exception.to_rust_string_lossy(try_catch);
            panic!("{exception_string}");
        }

        let ns = module.get_module_namespace();
        let _ = ns.to_object(try_catch).unwrap();
    }
}
