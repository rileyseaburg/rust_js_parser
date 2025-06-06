use js_processor::examples;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run examples without SSR features first
    examples::run_examples()?;
    
    #[cfg(feature = "ssr")]
    {
        use js_processor::actix_integration;
        
        println!("\n🌐 Starting Actix-Web server...");
        
        // Initialize V8 (required for SSR functionality)
        let platform = ssr_rs::v8::new_default_platform(0, false).make_shared();
        ssr_rs::v8::V8::initialize_platform(platform);
        ssr_rs::v8::V8::initialize();
        
        println!("JavaScript Processing Server with SWC and Actix-Web");
        println!("Features:");
        println!("- JavaScript/TypeScript parsing with SWC");
        println!("- React component compilation");
        println!("- Server-side rendering with V8");
        println!("- Actix-Web HTTP server");
        println!();
        
        // Start the server (this will block the thread)
        actix_integration::start_server().expect("Failed to start server");
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        println!("\n⚠️  SSR features disabled. To enable the Actix-Web server, compile with:");
        println!("   cargo run --features ssr");
        println!("\n🎉 JavaScript/TypeScript parsing and React compilation working!");
    }
    
    Ok(())
}