use anyhow::Result;
use swc_ecma_transforms_module::path::Resolver;
use std::sync::Arc;
use swc_atoms::Atom;
use swc_common::sync::Lrc;
use swc_common::{
    FileName, GLOBALS, Globals, Mark, SourceMap, comments::NoopComments, errors::Handler,
};
use swc_ecma_ast::EsVersion;
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};
use swc_ecma_transforms_module::common_js::{common_js};
use swc_ecma_transforms_react::{Options as ReactOptions, Runtime as ReactRuntime, react};
use swc_ecma_transforms_typescript::strip;
use std::borrow::Cow;

pub struct ReactCompiler {
    source_map: Lrc<SourceMap>,
    handler: Handler,
}

impl ReactCompiler {
    pub fn new() -> Self {
        let source_map: Lrc<SourceMap> = Default::default();
        let handler =
            Handler::with_emitter_writer(Box::new(std::io::stderr()), Some(source_map.clone()));

        Self {
            source_map,
            handler,
        }
    }

    pub fn compile_react_component(&self, code: &str, filename: Option<&str>) -> Result<String> {
        GLOBALS.set(&Globals::new(), || {
            let filename = filename.unwrap_or("component.tsx");
            let source_file = self.source_map.new_source_file(
                Lrc::new(FileName::Custom(filename.to_string())),
                code.to_string(),
            );

            let lexer = Lexer::new(
                Syntax::Typescript(TsSyntax {
                    tsx: true,
                    decorators: false,
                    dts: false,
                    no_early_errors: false,
                    disallow_ambiguous_jsx_like: true,
                }),
                EsVersion::Es2018,
                StringInput::from(&*source_file),
                None,
            );

            let mut parser = Parser::new_from(lexer);
            let module = parser
                .parse_module()
                .map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;

            let top_level_mark = Mark::new();
            let unresolved_mark = Mark::new();
            let mut program = swc_ecma_ast::Program::Module(module);

            let react_options = ReactOptions {
                runtime: Some(ReactRuntime::Classic),
                pragma: Some(Arc::new("React.createElement".to_string())),
                pragma_frag: Some(Arc::new("React.Fragment".to_string())),
                import_source: Some(Atom::from("react")),
                ..Default::default()
            };

            // Apply transforms in the correct order using tuples
            let transforms = (
                swc_ecma_transforms_base::resolver(unresolved_mark, top_level_mark, false),
                strip(top_level_mark, unresolved_mark),
                react(
                    self.source_map.clone(),
                    Some(&NoopComments),
                    react_options,
                    top_level_mark,
                    unresolved_mark,
                ),
            );

            // Apply all transforms at once
            program = program.apply(transforms);

            let common_js_transform = common_js(
                Resolver::default(),
                unresolved_mark,
                swc_ecma_transforms_module::common_js::Config::default(),
                swc_ecma_transforms_module::common_js::FeatureFlag::default(),
            );
            
            program = program.apply(common_js_transform);

            let module = match &program {
                swc_ecma_ast::Program::Module(m) => m.clone(),
                _ => unreachable!("Expected module"),
            };

            // Generate output code
            let mut buf = Vec::new();
            {
                let writer = JsWriter::new(self.source_map.clone(), "\n", &mut buf, None);
                let mut emitter = Emitter {
                    cfg: swc_ecma_codegen::Config::default(),
                    cm: self.source_map.clone(),
                    comments: None,
                    wr: writer,
                };
                emitter
                    .emit_module(&module)
                    .map_err(|e| anyhow::anyhow!("Emit error: {:?}", e))?;
            }

            let output = String::from_utf8(buf)
                .map_err(|e| anyhow::anyhow!("UTF-8 conversion error: {:?}", e))?;

            Ok(output)
        })
    }

    pub fn compile_react_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<String> {
        if !path.as_ref().exists() {
            return Err(anyhow::anyhow!("File not found: {:?}", path.as_ref()));
        }
        let code = std::fs::read_to_string(&path)?;
        let filename = path.as_ref().to_string_lossy();
        self.compile_react_component(&code, Some(&filename))
    }
}

impl Default for ReactCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple_react_component() {
        let compiler = ReactCompiler::new();
        let code = r#"
            import React from 'react';
            
            const MyComponent = () => {
                return <div>Hello, World!</div>;
            };
            
            export default MyComponent;
        "#;

        let result = compiler.compile_react_component(code, Some("component.tsx"));
        assert!(result.is_ok());

        let compiled = result.unwrap();
        // Should contain transformed JSX
        assert!(compiled.contains("React.createElement") || compiled.contains("jsx"));
    }

    #[test]
    fn test_compile_react_component_with_props() {
        let compiler = ReactCompiler::new();
        let code = r#"
            import React from 'react';
            
            interface Props {
                name: string;
                age: number;
            }
            
            const UserCard: React.FC<Props> = ({ name, age }) => {
                return (
                    <div className="user-card">
                        <h2>{name}</h2>
                        <p>Age: {age}</p>
                    </div>
                );
            };
            
            export default UserCard;
        "#;

        let result = compiler.compile_react_component(code, Some("user-card.tsx"));
        assert!(result.is_ok());
    }
}
