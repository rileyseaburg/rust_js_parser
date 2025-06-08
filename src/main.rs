use js_processor::actix_integration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_integration::start_server().await
}
