use ntex::web;

/// list of todo
#[utoipa::path(
  get,
  path="/todos",
  responses(
    (status=200,description="List of todo",body=[Todo]),
  ),
)]
#[web::get("/todos")]
pub async fn get_todos() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

/// create todo
#[utoipa::path(
    get,
    path="/todos",
    request_body = TodoPartial,
    responses(
      (status=201,description="Todo Created",body= Todo),
    ),
  )]
#[web::post("/todos")]
pub async fn create_todo() -> web::HttpResponse {
  web::HttpResponse::Created().finish()
}

/// get todo by id
#[utoipa::path(
    get,
    path="/todos/{id}",
    responses(
      (status=200,description="Todo found",body=Todo),
      (status=404,description="Todo not found",body=HttpError)
    ),
  )]
#[web::get("/todos/{id}")]
pub async fn get_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

/// update todo
#[utoipa::path(
    put,
    path="/todos/{id}",
    request_body= TodoPartial,
    responses(
      (status=200,description="Todo Update",body=Todo),
      (status=404,description="Todo not found",body = HttpError),
    ),
  )]
#[web::put("/todos/{id}")]
pub async fn update_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[utoipa::path(
    delete,
    path="/todos/{id}",
    responses(
      (status=200,description="Todo deleted",body=Todo),
      (status=404,description="Todo not found",body=HttpError),
    ),
  )]
#[web::delete("/todos/{id}")]
pub async fn delete_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) {
  cfg.service(get_todos);
  cfg.service(create_todo);
  cfg.service(get_todo);
  cfg.service(update_todo);
  cfg.service(delete_todo);
}
