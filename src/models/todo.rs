use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

///Todo model
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Todo {
  pub id: i32,
  pub title: String,
  pub completed: bool,
}

/// partial todo model
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TodoPartial {
  pub title: String,
}
