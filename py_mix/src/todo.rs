use anyhow::Result;
use core_sdk::models::todo::TodoModel;
use core_sdk::sdk::TodoSdk;
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

#[pyclass]
#[derive(Clone)]
pub struct Todo {
  sdk_handler: Arc<Mutex<TodoSdk>>,
}

impl Todo {
  pub fn new(sdk_handler: Arc<Mutex<TodoSdk>>) -> Self {
    Self { sdk_handler }
  }
}

#[pymethods]
impl Todo {
  #[pyo3(signature = (id))]
  fn get(&self, id: String) -> Result<TodoModel> {
    self.sdk_handler.lock().unwrap().get(id)
  }

  #[pyo3()]
  fn list(&self) -> Result<Vec<TodoModel>> {
    self.sdk_handler.lock().unwrap().list()
  }

  #[pyo3(signature = (title, completed))]
  fn add(&self, title: String, completed: Option<bool>) -> Result<TodoModel> {
    self.sdk_handler.lock().unwrap().add(title, completed)
  }

  #[pyo3(signature = (id, title, completed))]
  fn edit(&self, id: String, title: String, completed: Option<bool>) -> Result<TodoModel> {
    self.sdk_handler.lock().unwrap().edit(id, title, completed)
  }

  #[pyo3(signature = (id))]
  fn delete(&self, id: String) -> Result<()> {
    self.sdk_handler.lock().unwrap().delete(id)
  }
}
