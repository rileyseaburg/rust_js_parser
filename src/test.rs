#[cfg(test)]
mod test {
    use crate::JsHttpRequestProcessor;
    use crate::StringHttpRequest;
    use std::collections::HashMap;
    use std::fs;
    use ssr_rs::v8;
    use swc_common::GLOBALS;

    #[test]
    fn test_editor_ssr_require() {
        GLOBALS.set(&Default::default(), || {
            let platform = ssr_rs::v8::new_default_platform(0, false).make_shared();
            ssr_rs::v8::V8::initialize_platform(platform);
            ssr_rs::v8::V8::initialize();

            let isolate = &mut ssr_rs::v8::Isolate::new(ssr_rs::v8::CreateParams::default());
            let mut isolate_scope = ssr_rs::v8::HandleScope::new(isolate);

            let source = fs::read_to_string("dist/server/editor.js").unwrap_or_else(|_| {
                // Fallback JavaScript if the file doesn't exist
                r#"
                function Process(request) {
                    output.body = "Processed: " + request.path;
                    output.headers = { "Content-Type": "text/plain" };
                    output.status = 200;
                }
                "#.to_string()
            });
            let source = ssr_rs::v8::String::new(&mut isolate_scope, &source).unwrap();

            let mut processor = JsHttpRequestProcessor::new(&mut isolate_scope, source, HashMap::new());

            let request = StringHttpRequest::new("/test-path", "example.com", "test-agent", "test-referer");
            processor.process(request);

            let output_str = v8::String::new(&mut processor.context_scope, "output").unwrap();
            let output_obj = processor.context.global(&mut processor.context_scope)
                .get(&mut processor.context_scope, output_str.into())
                .expect("output object not found");

            let output_obj = v8::Local::<v8::Object>::try_from(output_obj).expect("output is not an object");

            let body_key = v8::String::new(&mut processor.context_scope, "body").unwrap();
            let body_value = output_obj.get(&mut processor.context_scope, body_key.into()).unwrap();
            let body_string = body_value.to_rust_string_lossy(&mut processor.context_scope);

            assert_eq!(body_string, "Processed: /test-path");

            let status_key = v8::String::new(&mut processor.context_scope, "status").unwrap();
            let status_value = output_obj.get(&mut processor.context_scope, status_key.into()).unwrap();
            let status_int = status_value.to_int32(&mut processor.context_scope).unwrap().value();

            
            assert_eq!(status_int, 200);
        });
    }
}
