use ssr_rs::v8;

pub fn create_script_origin<'s>(
    scope: &mut v8::HandleScope<'s>,
    filename: &str,
    is_module: bool,
) -> v8::ScriptOrigin<'s> {
    let name = v8::String::new(scope, filename).unwrap();

    v8::ScriptOrigin::new(
        scope,
        name.into(),
        0,
        0,
        false,
        0,
        None,
        false,
        false,
        is_module,
        None,
    )
}
