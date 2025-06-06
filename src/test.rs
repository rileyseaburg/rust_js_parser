use super::*;
use std::collections::HashMap;
use std::fs;

#[test]
fn test_editor_ssr_require() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
    let mut isolate_scope = v8::HandleScope::new(isolate);

    let source = fs::read_to_string("dist/server/editor.js").expect("failed to read editor.js");
    let source = v8::String::new(&mut isolate_scope, &source).unwrap();

    let _processor = JsHttpRequestProcessor::new(&mut isolate_scope, source, HashMap::new());
}
