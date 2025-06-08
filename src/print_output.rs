use ssr_rs::v8;
use super::JsHttpRequestProcessor;

impl<'s, 'i> JsHttpRequestProcessor<'s, 'i>
where
    's: 'i,
{
    /// Prints the output.
    pub fn print_output(&mut self) {
        let scope = &mut v8::HandleScope::new(&mut *self.context_scope);
        let key = v8::String::new(scope, "output").unwrap();
        let output = self
            .context
            .global(scope)
            .get(scope, key.into())
            .unwrap()
            .to_object(scope)
            .unwrap();

        let props = output
            .get_property_names(scope, v8::GetPropertyNamesArgsBuilder::new().build())
            .unwrap();
        for i in 0..props.length() {
            let key = props.get_index(scope, i).unwrap();
            let value = output.get(scope, key).unwrap();

            let key = key.to_rust_string_lossy(scope);
            let value = value.to_rust_string_lossy(scope);

            println!("foss {key}: {value}");
        }
    }
}
