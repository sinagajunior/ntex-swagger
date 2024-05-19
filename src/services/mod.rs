use ntex::web;

pub mod todo;
pub mod openapi;

pub async fn default() -> web::HttpResponse {
  web::HttpResponse::NotFound().finish()
}
