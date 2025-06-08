use super::{log_callback, require_callback, JsHttpRequestProcessor};
use crate::send_wrapper::SendWrapper;
use ssr_rs::v8;
use std::collections::HashMap;
use std::convert::TryFrom;
use swc::Compiler;
use swc_common::errors::{ColorConfig, Handler};
use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_ecma_parser::Syntax;
use swc_ecma_parser::TsSyntax;

impl<'s, 'i> JsHttpRequestProcessor<'s, 'i>
where
    's: 'i,
{
    /// Creates a scriptable HTTP request processor.
    pub fn new(
        isolate_scope: &'i mut v8::HandleScope<'s, ()>,
        source: v8::Local<'s, v8::String>,
        options: HashMap<String, String>,
    ) -> Self {
        let global = v8::ObjectTemplate::new(isolate_scope);
        global.set(
            v8::String::new(isolate_scope, "log").unwrap().into(),
            v8::FunctionTemplate::new(isolate_scope, log_callback).into(),
        );
        global.set(
            v8::String::new(isolate_scope, "require").unwrap().into(),
            v8::FunctionTemplate::new(isolate_scope, require_callback).into(),
        );

        let context = v8::Context::new(
            isolate_scope,
            v8::ContextOptions {
                global_template: Some(global),
                ..Default::default()
            },
        );
        let mut context_scope = v8::ContextScope::new(isolate_scope, context);

        let request_template = v8::ObjectTemplate::new(&mut context_scope);
        request_template.set_internal_field_count(1);

        // make it global
        let request_template = v8::Global::new(&mut context_scope, request_template);

        let mut self_ = JsHttpRequestProcessor {
            context: unsafe { SendWrapper::new(context) },
            context_scope: unsafe { SendWrapper::new(context_scope) },
            process_fn: None,
            request_template,
            _map_template: None,
        };

        // loads options and output
        let options = self_.wrap_map(options);
        let options_str = v8::String::new(&mut *self_.context_scope, "options").unwrap();
        self_.context.global(&mut *self_.context_scope).set(
            &mut *self_.context_scope,
            options_str.into(),
            options.into(),
        );

        let output = v8::Object::new(&mut *self_.context_scope);
        let output_str = v8::String::new(&mut *self_.context_scope, "output").unwrap();
        self_.context.global(&mut *self_.context_scope).set(
            &mut *self_.context_scope,
            output_str.into(),
            output.into(),
        );

        // execute script
        let cm: Lrc<SourceMap> = Default::default();
        let handler = Handler::with_emitter_writer(
            Box::new(std::io::stderr()),
            Some(cm.clone()),
        );
        let compiler = Compiler::new(cm.clone());
        let fm = cm.new_source_file(
            swc_common::FileName::Custom("in.js".into()).into(),
            source.to_rust_string_lossy(&mut *self_.context_scope),
        );
        let transformed = compiler
            .process_js_file(
                fm,
                &handler,
                &swc::config::Options {
                    config: swc::config::Config {
                        jsc: swc::config::JscConfig {
                            syntax: Some(Syntax::Typescript(TsSyntax {
                                tsx: true,
                                decorators: true,
                                dts: false,
                                no_early_errors: false,
                                disallow_ambiguous_jsx_like: true,
                            })),
                            target: Some(swc_ecma_ast::EsVersion::Es5),
                            ..Default::default()
                        },
                        module: Some(swc::config::ModuleConfig::Es6(Default::default())),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .unwrap();

        let transformed_source =
            v8::String::new(&mut *self_.context_scope, &transformed.code).unwrap();
        if transformed.code.contains("import") || transformed.code.contains("export") {
            self_.execute_module(transformed_source);
        } else {
            self_.execute_script(transformed_source);
        }

        let process_str = v8::String::new(&mut *self_.context_scope, "Process").unwrap();
        let process_fn = self_
            .context
            .global(&mut *self_.context_scope)
            .get(&mut *self_.context_scope, process_str.into())
            .expect("missing function Process");

        let process_fn =
            v8::Local::<v8::Function>::try_from(process_fn).expect("function expected");
        self_.process_fn = Some(process_fn);

        self_
    }
}
