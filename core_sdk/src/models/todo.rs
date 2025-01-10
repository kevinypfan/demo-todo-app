use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg_attr(feature = "python", pyclass(get_all))]
#[derive(Serialize, Deserialize, Debug)]
pub struct TodoModel {
  pub title: String,
  pub completed: Option<bool>,
  #[serde(rename = "_id")]
  pub id: Option<String>,
  #[serde(rename = "createdAt")]
  pub created_at: Option<String>,
  #[serde(rename = "updatedAt")]
  pub updated_at: Option<String>,
  #[serde(rename = "__v", skip_serializing)]
  pub version: Option<i32>,
}

#[cfg(feature = "python")]
#[pymethods]
impl TodoModel {
  fn __str__(&self) -> String {
    format!("{:#?}", self)
  }

  fn __repr__(&self) -> String {
    format!("{:#?}", self)
  }
}

impl TodoModel {
  pub fn new(title: String, completed: Option<bool>) -> Self {
    Self {
      title,
      completed,
      id: None,
      created_at: None,
      updated_at: None,
      version: None,
    }
  }

  pub fn set_title(&mut self, title: String) -> &mut TodoModel {
    self.title = title;
    self
  }

  pub fn set_completed(&mut self, completed: bool) -> &mut TodoModel {
    self.completed = Some(completed);
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json;

  #[test]
  fn test_todo_deserialize() {
    let json_data = r#"
        {
            "title": "Learn Docker",
            "completed": false,
            "_id": "677fffd6e7bbb603a1a5c67e",
            "createdAt": "2025-01-09T16:56:54.714Z",
            "updatedAt": "2025-01-09T16:56:54.714Z",
            "__v": 0
        }"#;

    let todo: TodoModel = serde_json::from_str(json_data).unwrap();

    assert_eq!(todo.title, "Learn Docker");
    assert_eq!(todo.completed, Some(false));
    assert_eq!(todo.id, Some("677fffd6e7bbb603a1a5c67e".to_string()));
    assert_eq!(
      todo.created_at,
      Some("2025-01-09T16:56:54.714Z".to_string())
    );
    assert_eq!(
      todo.updated_at,
      Some("2025-01-09T16:56:54.714Z".to_string())
    );
    assert_eq!(todo.version, Some(0));
  }

  #[test]
  fn test_todo_serialize() {
    let todo = TodoModel {
      title: "Learn Docker".to_string(),
      completed: Some(false),
      id: Some("677fffd6e7bbb603a1a5c67e".to_string()),
      created_at: Some("2025-01-09T16:56:54.714Z".to_string()),
      updated_at: Some("2025-01-09T16:56:54.714Z".to_string()),
      version: Some(0),
    };

    let serialized = serde_json::to_string(&todo).unwrap();

    let expected_json = r#"{"title":"Learn Docker","completed":false,"id":"677fffd6e7bbb603a1a5c67e","created_at":"2025-01-09T16:56:54.714Z","updated_at":"2025-01-09T16:56:54.714Z"}"#;
    assert_eq!(serialized, expected_json);
  }
}
