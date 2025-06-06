use anyhow::Result;
use swc_common::errors::Handler;
use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap};
use swc_ecma_parser::{parse_file_as_module, Syntax, TsSyntax};
use swc_ecma_ast::{Module, EsVersion};
use std::path::Path;

/// A basic JavaScript parser using SWC
pub struct JavaScriptParser {
    source_map: Lrc<SourceMap>,
    handler: Handler,
}

impl JavaScriptParser {
    /// Create a new JavaScript parser
    pub fn new() -> Self {
        let source_map: Lrc<SourceMap> = Default::default();
        let handler = Handler::with_emitter_writer(
            Box::new(std::io::stderr()),
            Some(source_map.clone()),
        );
        
        Self {
            source_map,
            handler,
        }
    }
    
    /// Parse JavaScript/TypeScript code from a string
    pub fn parse_code(&self, code: &str, filename: Option<&str>) -> Result<Module> {
        let filename = filename.unwrap_or("input.js");
        let source_file = self.source_map.new_source_file(
            FileName::Custom(filename.to_string()).into(),
            code.to_string(),
        );
        
        let syntax = Syntax::Typescript(TsSyntax {
            tsx: true,
            decorators: true,
            dts: filename.ends_with(".d.ts"),
            no_early_errors: false,
            disallow_ambiguous_jsx_like: true,
        });
        
        let module = parse_file_as_module(
            &source_file,
            syntax,
            EsVersion::Es2022,
            None,
            &mut vec![],
        ).map_err(|err| anyhow::anyhow!("Parse error: {:?}", err))?;
        
        Ok(module)
    }
    
    /// Parse JavaScript/TypeScript code from a file
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<Module> {
        let code = std::fs::read_to_string(&path)?;
        let filename = path.as_ref().to_string_lossy();
        self.parse_code(&code, Some(&filename))
    }
}

impl Default for JavaScriptParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_javascript() {
        let parser = JavaScriptParser::new();
        let code = r#"
            function hello() {
                console.log("Hello, World!");
            }
            hello();
        "#;
        
        let result = parser.parse_code(code, Some("test.js"));
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_parse_typescript() {
        let parser = JavaScriptParser::new();
        let code = r#"
            interface User {
                name: string;
                age: number;
            }
            
            function greet(user: User): string {
                return `Hello, ${user.name}!`;
            }
        "#;
        
        let result = parser.parse_code(code, Some("test.ts"));
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_parse_react_component() {
        let parser = JavaScriptParser::new();
        let code = r#"
            import React from 'react';
            
            interface Props {
                name: string;
            }
            
            const MyComponent: React.FC<Props> = ({ name }) => {
                return <div>Hello, {name}!</div>;
            };
            
            export default MyComponent;
        "#;
        
        let result = parser.parse_code(code, Some("component.tsx"));
        assert!(result.is_ok());
    }
}