use ntex::web;
mod services;
mod error;
mod models;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| {
    web::App::new()
      .configure(services::openapi::ntex_config)
      .configure(services::todo::ntex_config)
      .default_service(web::route().to(services::default))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
