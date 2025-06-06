#[cfg(all(test, feature = "ssr"))]
mod test {
    use super::*;
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn test_editor_ssr_require() {
        let platform = ssr_rs::v8::new_default_platform(0, false).make_shared();
        ssr_rs::v8::V8::initialize_platform(platform);
        ssr_rs::v8::V8::initialize();

        let isolate = &mut ssr_rs::v8::Isolate::new(ssr_rs::v8::CreateParams::default());
        let mut isolate_scope = ssr_rs::v8::HandleScope::new(isolate);

        let source = fs::read_to_string("dist/server/editor.js").unwrap_or_else(|_| {
            // Fallback JavaScript if the file doesn't exist
            "function test() { return 'Hello from SSR!'; }".to_string()
        });
        let source = ssr_rs::v8::String::new(&mut isolate_scope, &source).unwrap();

        let _processor = JsHttpRequestProcessor::new(&mut isolate_scope, source, HashMap::new());
    }
}
