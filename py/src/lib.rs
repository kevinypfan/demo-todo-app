mod todo;
mod callback;

use crate::todo::Todo;
use anyhow::Result;
use core_sdk::sdk::TodoSdk;
use pyo3::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
use pyo3::exceptions::PyException;
use sysinfo::{Disks, System};
use url::Url;
use core_sdk::transport::websocket::WebSocketConnection;
use crate::callback::{register_callback, Callbacks};

#[pyclass(subclass)]
struct CoreSdk {
  sdk_handler: Arc<Mutex<TodoSdk>>,
  ws_handler: Option<Arc<WebSocketConnection>>,
  callbacks: Arc<Mutex<Callbacks>>,
  callback_thread: Option<std::thread::JoinHandle<()>>,
  #[pyo3(get)]
  pub todo: Todo,
}

#[pymethods]
impl CoreSdk {
  #[new]
  #[pyo3(signature = (api_url = None))]
  pub fn new(api_url: Option<String>) -> Result<Self> {
    let todo_sdk = TodoSdk::new(api_url);
    let sdk_handler = Arc::new(Mutex::new(todo_sdk));
    let callbacks = Arc::new(Mutex::new(Callbacks::default()));
    Ok(Self {
      sdk_handler: sdk_handler.clone(),
      todo: Todo::new(sdk_handler.clone()),
      callback_thread: None,
      ws_handler: None,
      callbacks,
    })
  }

  fn connect_websocket(&mut self) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    let ws_url = Url::parse(&self.sdk_handler.lock().unwrap().get_ws_url())
      .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))?;
    let connection = WebSocketConnection::new(&ws_url, None, tx.clone())
      .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))?;

    let callback_thread = {
      let callbacks = self.callbacks.clone();
      std::thread::spawn(move || {
        register_callback(
          Arc::new(Mutex::new(rx)),
          callbacks.clone(),
        );
      })
    };
    self.callback_thread = Some(callback_thread);
    self.ws_handler = Some(Arc::new(connection));

    Ok(())
  }

  fn set_on_changed(&self, py: Python<'_>, callback: PyObject) {
    if callback.is_none(py) {
      self.callbacks.lock().unwrap().changed = None;
    } else {
      self.callbacks.lock().unwrap().changed = Some(callback);
    }
  }

  fn set_on_error(&self, py: Python<'_>, callback: PyObject) {
    if callback.is_none(py) {
      self.callbacks.lock().unwrap().error = None;
    } else {
      self.callbacks.lock().unwrap().error = Some(callback);
    }
  }

  fn set_on_disconnected(&self, py: Python<'_>, callback: PyObject) {
    if callback.is_none(py) {
      self.callbacks.lock().unwrap().disconnected = None;
    } else {
      self.callbacks.lock().unwrap().disconnected = Some(callback);
    }
  }

}

#[pyclass]
struct Util {}

#[pymethods]
impl Util {
  #[staticmethod]
  fn show_computer_info() -> Result<()> {
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

    // We display all disks' information:
    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
      println!("{disk:?}");
    }

    Ok(())
  }
}

/// A Python module implemented in Rust.
#[pymodule]
fn todo_app(m: &Bound<'_, PyModule>) -> PyResult<()> {
  m.add_class::<CoreSdk>()?;
  m.add_class::<Util>()?;
  Ok(())
}
