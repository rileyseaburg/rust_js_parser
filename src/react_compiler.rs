use anyhow::Result;
use swc::Compiler;
use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap, Globals, GLOBALS};
use swc_common::errors::Handler;
use swc_ecma_parser::{Syntax, TsSyntax};
use swc_ecma_ast::EsVersion;

/// A React component compiler using SWC
pub struct ReactCompiler {
    compiler: Compiler,
    source_map: Lrc<SourceMap>,
    handler: Handler,
}

impl ReactCompiler {
    /// Create a new React compiler
    pub fn new() -> Self {
        let source_map: Lrc<SourceMap> = Default::default();
        let handler = Handler::with_emitter_writer(
            Box::new(std::io::stderr()),
            Some(source_map.clone()),
        );
        let compiler = Compiler::new(source_map.clone());
        
        Self {
            compiler,
            source_map,
            handler,
        }
    }
    
    /// Compile React JSX/TSX code to JavaScript
    pub fn compile_react_component(&self, code: &str, filename: Option<&str>) -> Result<String> {
        swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
            let filename = filename.unwrap_or("component.tsx");
            let source_file = self.source_map.new_source_file(
                FileName::Custom(filename.to_string()).into(),
                code.to_string(),
            );
            
            let options = swc::config::Options {
                config: swc::config::Config {
                    jsc: swc::config::JscConfig {
                        syntax: Some(Syntax::Typescript(TsSyntax {
                            tsx: true,
                            decorators: true,
                            dts: false,
                            no_early_errors: false,
                            disallow_ambiguous_jsx_like: true,
                        })),
                        target: Some(EsVersion::Es2018),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            };
            
            let output = self.compiler.process_js_file(
                source_file,
                &self.handler,
                &options,
            ).map_err(|err| anyhow::anyhow!("Compilation error: {:?}", err))?;
            
            Ok(output.code)
        })
    }
    
    /// Compile a React component from a file
    pub fn compile_react_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<String> {
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