use crate::api::ApiClient;
use crate::constants::REST_BASE_URL;
use crate::models::todo::TodoModel;
use anyhow::{anyhow, Result};
use log::info;
use serde::Serialize;
use serde_json::{from_str, json};
use ureq::Response;

#[derive(Debug, Clone)]
pub struct TodoSdk {
  api_client: ApiClient,
}

impl TodoSdk {
  pub fn new(base_url: Option<String>) -> Self {
    let base_url = base_url.unwrap_or(REST_BASE_URL.to_string());
    let api_client = ApiClient::new(base_url);
    Self { api_client }
  }

  pub fn list(&self) -> Result<Vec<TodoModel>> {
    let response = self.api_client.get("/todos", None)?;
    let body = response.into_string().map_err(|e| anyhow!(e))?;
    let todos: Vec<TodoModel> = from_str(&body).map_err(|e| anyhow!(e))?;
    Ok(todos)
  }
  pub fn get(&self, id: String) -> Result<TodoModel> {
    let response = self.api_client.get(&format!("{}/{}", "/todos", id), None)?;
    let body = response.into_string().map_err(|e| anyhow!(e))?;
    let todo: TodoModel = from_str(&body).map_err(|e| anyhow!(e))?;
    Ok(todo)
  }
  pub fn add(&self, title: String, completed: Option<bool>) -> Result<TodoModel> {
    let todo = TodoModel::new(title, completed);
    let response = self
      .api_client
      .post("/todos", None, Some(serde_json::to_string(&todo)?))?;
    let body = response.into_string().map_err(|e| anyhow!(e))?;
    info!("{:?}", body);
    let created_todo: TodoModel = from_str(&body).map_err(|e| anyhow!(e))?;
    Ok(created_todo)
  }

  pub fn edit(&self, id: String, title: String, completed: Option<bool>) -> Result<TodoModel> {
    let mut todo = self.get(id.clone())?;
    todo.set_title(title);
    if let Some(completed) = completed {
      todo.set_completed(completed);
    }

    let response = self.api_client.put(
      &format!("/todos/{}", id),
      None,
      Some(serde_json::to_string(&todo)?),
    )?;

    let body = response.into_string().map_err(|e| anyhow!(e))?;
    info!("{:?}", body);
    let updated_todo: TodoModel = serde_json::from_str(&body).map_err(|e| anyhow!(e))?;
    Ok(updated_todo)
  }

  pub fn delete(&self, id: String) -> Result<()> {
    self
      .api_client
      .delete(&format!("/todos/{}", id), None)
      .map_err(|e| anyhow::anyhow!(e))?;

    Ok(())
  }
}
