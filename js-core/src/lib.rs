mod todo;
mod model;
use napi::{Env, Result};

#[macro_use]
extern crate napi_derive;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use napi::bindgen_prelude::Reference;
use core_sdk::sdk::TodoSdk;
use crate::todo::JsTodo;

#[napi(js_name = "CoreSdk")]
struct JsCoreSDK {
  sdk_handler: Arc<Mutex<TodoSdk>>,
  todo: Reference<JsTodo>,
}

#[napi]
impl JsCoreSDK {

  #[napi(constructor)]
  pub fn new(env: Env) -> Self {
    let todo_sdk = TodoSdk::new(None);
    let sdk_handler = Arc::new(Mutex::new(todo_sdk));
    JsCoreSDK {
      sdk_handler: sdk_handler.clone(),
      todo: JsTodo::into_reference(JsTodo::new(sdk_handler.clone()), env).unwrap(),
    }
  }

  #[napi(getter)]
  pub fn todo(&self, env: Env) -> Result<Reference<JsTodo>> {
    Ok(self.todo.clone(env).unwrap())
  }
}

