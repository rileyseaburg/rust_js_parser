# rust_js_parser

A high-performance JavaScript/TypeScript processing library built with Rust, featuring SWC integration and Actix-Web server-side rendering capabilities.

## Features

✅ **JavaScript/TypeScript Parsing**: Parse modern JavaScript and TypeScript code using SWC
✅ **React Component Compilation**: Compile React JSX/TSX components to JavaScript
✅ **Server-Side Rendering**: Optional V8-based SSR capabilities using ssr_rs
✅ **Actix-Web Integration**: HTTP server for serving rendered components
✅ **Type Safety**: Full TypeScript support with interface definitions
✅ **Modern Syntax**: ES2022 support with decorators and advanced features

## Quick Start

### Basic Usage (Without SSR)

```bash
# Run examples showcasing parsing and compilation
cargo run --no-default-features

# Run tests
cargo test --no-default-features
```

### With SSR Features

```bash  
# Enable full SSR capabilities (requires V8 compilation)
cargo run --features ssr

# Run all tests including SSR
cargo test --features ssr
```

## Examples

### JavaScript Parsing

```rust
use js_processor::js_parser::JavaScriptParser;

let parser = JavaScriptParser::new();
let code = r#"
    function greet(name) {
        return `Hello, ${name}!`;
    }
"#;

let ast = parser.parse_code(code, Some("example.js"))?;
println!("✅ JavaScript parsing successful!");
```

### TypeScript Parsing

```rust
let ts_code = r#"
    interface User {
        id: number;
        name: string;
    }
    
    function greetUser(user: User): string {
        return `Hello, ${user.name}!`;
    }
"#;

let ast = parser.parse_code(ts_code, Some("example.ts"))?;
println!("✅ TypeScript parsing successful!");
```

### React Component Compilation

```rust
use js_processor::react_compiler::ReactCompiler;

let compiler = ReactCompiler::new();
let jsx_code = r#"
    import React from 'react';
    
    const MyComponent = ({ title }) => {
        return <h1>{title}</h1>;
    };
"#;

let compiled = compiler.compile_react_component(jsx_code, Some("component.tsx"))?;
println!("Compiled: {}", compiled);
// Output: const MyComponent = ({ title }) => { return /*#__PURE__*/ React.createElement("h1", null, title); };
```

### HTTP Request Processing

```rust
use js_processor::{StringHttpRequest, ssr::http_request::SimpleHttpRequest};

let request = StringHttpRequest::new("/api/users", "example.com", "Mozilla/5.0", "https://example.com");

println!("Path: {}", request.path());
println!("Host: {}", request.host());
```

## Architecture

- **`js_parser`**: JavaScript/TypeScript parsing using SWC
- **`react_compiler`**: JSX/TSX to JavaScript compilation
- **`ssr`**: Server-side rendering utilities and HTTP request abstractions  
- **`actix_integration`**: Actix-Web server integration
- **`examples`**: Demonstration code and usage examples

## Dependencies

### Core Dependencies
- `swc`: JavaScript/TypeScript parsing and transformation
- `swc_common`: Common utilities for SWC
- `swc_ecma_parser`: ECMAScript parsing capabilities
- `swc_ecma_ast`: AST definitions
- `anyhow`: Error handling
- `serde`: Serialization framework

### Optional SSR Dependencies (feature = "ssr")
- `ssr_rs`: V8-based server-side rendering
- `actix-web`: Web framework
- `tokio`: Async runtime

## Testing

The project includes comprehensive tests for all major functionality:

```bash
# Test basic parsing functionality
cargo test --no-default-features js_parser

# Test React compilation
cargo test --no-default-features react_compiler  

# Test HTTP request handling
cargo test --no-default-features simple_tests

# Test everything including SSR (requires V8)
cargo test --features ssr
```

## File Examples

See the `examples/` directory for sample files:
- `examples/test.js` - Basic JavaScript
- `examples/typescript-example.ts` - TypeScript with interfaces
- `examples/react-component.tsx` - React component with TypeScript

## Performance

- Fast parsing using SWC's Rust-based parser
- Efficient memory usage with zero-copy string handling
- Optional V8 integration for JavaScript execution
- Async/await support with Tokio runtime

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test --no-default-features`
5. Submit a pull request

## License

This project is open source. See LICENSE file for details.
