use serde::{Deserialize, Deserializer, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "js")]
use napi_derive::napi;

#[cfg_attr(feature = "python", pyclass(get_all))]
#[cfg_attr(feature = "js", napi(object))]
#[derive(Serialize, Deserialize, Debug)]
pub struct EventObj {
  #[serde(rename = "event")]
  pub event_name: String,

  #[serde(rename = "data")]
  pub data: String,
}

#[cfg(feature = "python")]
#[pymethods]
impl EventObj {
  fn __str__(&self) -> String {
    format!("{:#?}", self)
  }

  fn __repr__(&self) -> String {
    format!("{:#?}", self)
  }
}
