use serde::Serialize;
use core_sdk::models::todo::TodoModel;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[napi(object, js_name = "TodoModel")]
pub struct JsTodoModel {
  pub id: Option<String>,
  pub title: String,
  pub completed: Option<bool>,
  pub created_at: Option<String>,
  pub updated_at: Option<String>,
  pub version: Option<i32>,
}

impl From<TodoModel> for JsTodoModel {
  fn from(a: TodoModel) -> Self {
    Self {
      id: a.id,
      title: a.title,
      completed: a.completed,
      created_at: a.created_at,
      updated_at: a.updated_at,
      version: a.version,
    }
  }
}

impl Into<TodoModel> for JsTodoModel {
  fn into(self) -> TodoModel {
    TodoModel {
      id: self.id,
      title: self.title,
      completed: self.completed,
      created_at: self.created_at,
      updated_at: self.updated_at,
      version: self.version,
    }
  }
}
