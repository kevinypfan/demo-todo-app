mod todo;

use crate::todo::Todo;
use anyhow::Result;
use core_sdk::sdk::TodoSdk;
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use sysinfo::{Components, Disks, Networks, System};

#[pyclass(subclass)]
struct CoreSdk {
  sdk_handler: Arc<Mutex<TodoSdk>>,
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

    Ok(Self {
      sdk_handler: sdk_handler.clone(),
      todo: Todo::new(sdk_handler.clone()),
    })
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
#[pyo3(name = "_todo_sdk")]
fn todo_app(m: &Bound<'_, PyModule>) -> PyResult<()> {
  m.add_class::<CoreSdk>()?;
  m.add_class::<Util>()?;
  Ok(())
}
