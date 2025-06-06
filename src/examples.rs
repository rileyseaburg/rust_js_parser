use crate::{js_parser::JavaScriptParser, react_compiler::ReactCompiler};
use anyhow::Result;

/// Example demonstrating JavaScript parsing functionality
pub fn example_js_parsing() -> Result<()> {
    println!("=== JavaScript Parser Example ===");
    
    let parser = JavaScriptParser::new();
    
    // Parse simple JavaScript
    let js_code = r#"
        function add(a, b) {
            return a + b;
        }
        
        const result = add(5, 3);
        console.log(result);
    "#;
    
    println!("Parsing JavaScript code:");
    println!("{}", js_code);
    
    match parser.parse_code(js_code, Some("example.js")) {
        Ok(_module) => {
            println!("✅ JavaScript parsing successful!");
        }
        Err(e) => {
            println!("❌ JavaScript parsing failed: {}", e);
            return Err(e);
        }
    }
    
    // Parse TypeScript
    let ts_code = r#"
        interface User {
            id: number;
            name: string;
        }
        
        function greetUser(user: User): string {
            return `Hello, ${user.name}!`;
        }
    "#;
    
    println!("\nParsing TypeScript code:");
    println!("{}", ts_code);
    
    match parser.parse_code(ts_code, Some("example.ts")) {
        Ok(_module) => {
            println!("✅ TypeScript parsing successful!");
        }
        Err(e) => {
            println!("❌ TypeScript parsing failed: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

/// Example demonstrating React component compilation
pub fn example_react_compilation() -> Result<()> {
    println!("\n=== React Compiler Example ===");
    
    let compiler = ReactCompiler::new();
    
    let react_code = r#"
        import React from 'react';
        
        interface Props {
            title: string;
        }
        
        const MyComponent: React.FC<Props> = ({ title }) => {
            return <h1>{title}</h1>;
        };
        
        export default MyComponent;
    "#;
    
    println!("Compiling React component:");
    println!("{}", react_code);
    
    match compiler.compile_react_component(react_code, Some("component.tsx")) {
        Ok(compiled) => {
            println!("✅ React compilation successful!");
            println!("Compiled output (first 200 chars):");
            let preview = if compiled.len() > 200 {
                format!("{}...", &compiled[..200])
            } else {
                compiled
            };
            println!("{}", preview);
        }
        Err(e) => {
            println!("❌ React compilation failed: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

/// Run all examples
pub fn run_examples() -> Result<()> {
    println!("🚀 JavaScript Processing Examples with SWC and Actix-Web\n");
    
    example_js_parsing()?;
    example_react_compilation()?;
    
    println!("\n✅ All examples completed successfully!");
    Ok(())
}