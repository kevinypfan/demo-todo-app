use crate::model::JsTodoModel;
use anyhow::{Context};
use core_sdk::models::todo::TodoModel;
use core_sdk::sdk::TodoSdk;
use napi::bindgen_prelude::*;
use napi::{bindgen_prelude::*, Error};
use std::sync::{Arc, Mutex};
use anyhow::Result;

#[napi(js_name = "Todo")]
pub struct JsTodo {
  sdk_handler: Option<Arc<Mutex<TodoSdk>>>,
}

impl JsTodo {
  pub fn new(sdk_handler: Arc<Mutex<TodoSdk>>) -> Self {
    JsTodo {
      sdk_handler: Some(sdk_handler),
    }
  }
}

#[napi]
impl JsTodo {

  #[napi(constructor)]
  pub fn new_empty() -> Result<Self> {
    Ok(JsTodo { sdk_handler: None })
  }
  #[napi]
  pub fn get(&self, id: String) -> Result<TodoModel> {
    let binding = self.sdk_handler.clone().unwrap();
    let sdk_handler = binding.lock().unwrap();

    sdk_handler.get(id)
  }

  #[napi]
  pub fn list(&self) -> Result<Vec<TodoModel>> {
    let binding = self.sdk_handler.clone().unwrap();
    let sdk_handler = binding.lock().unwrap();
    sdk_handler.list()
  }

  #[napi]
  pub fn add(&self, title: String, completed: Option<bool>) -> Result<TodoModel> {
    let binding = self.sdk_handler.clone().unwrap();
    let sdk_handler = binding.lock().unwrap();
    sdk_handler.add(title, completed)
  }

  #[napi]
  pub fn edit(&self, id: String, title: String, completed: Option<bool>) -> Result<TodoModel> {
    let binding = self.sdk_handler.clone().unwrap();
    let sdk_handler = binding.lock().unwrap();
    sdk_handler.edit(id, title, completed)
  }

  #[napi]
  pub fn delete(&self, id: String) -> Result<()> {
    let binding = self.sdk_handler.clone().unwrap();
    let sdk_handler = binding.lock().unwrap();
    sdk_handler.delete(id)
  }
}
