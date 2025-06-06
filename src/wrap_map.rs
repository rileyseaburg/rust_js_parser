use std::collections::HashMap;
use ssr_rs::v8;
use super::JsHttpRequestProcessor;

impl<'s, 'i> JsHttpRequestProcessor<'s, 'i>
where
    's: 'i,
{
    pub fn wrap_map(&mut self, options: HashMap<String, String>) -> v8::Local<'s, v8::Object> {
        // TODO: wrap map, not convert into Object
        let scope = &mut self.context_scope;
        let result = v8::Object::new(scope);

        for (key, value) in options {
            let key = v8::String::new(scope, &key).unwrap().into();
            let value = v8::String::new(scope, &value).unwrap().into();
            result.set(scope, key, value);
        }

        result
    }
}
