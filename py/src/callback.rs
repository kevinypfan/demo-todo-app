use core_sdk::transport::websocket::StatusMessage;
use log::info;
use pyo3::prelude::*;
use std::sync::{mpsc::Receiver, Arc, Mutex};
use std::thread::sleep;

#[derive(Debug, Default)]
pub struct Callbacks {
  pub(crate) changed: Option<PyObject>,
  pub(crate) error: Option<PyObject>,
  pub(crate) disconnected: Option<PyObject>,
}

pub fn register_callback(
  rx: Arc<Mutex<Receiver<StatusMessage>>>,
  callbacks: Arc<Mutex<Callbacks>>,
) {
  let rx = rx.clone();
  let callbacks = callbacks.clone();
  loop {
    let message = rx.lock().unwrap().recv();
    Python::with_gil(|py| match message {
      Ok(sure_message) => match sure_message {
        StatusMessage::Connected => {}
        StatusMessage::MessageReceived(msg) => {
          info!("ws={}", msg);
          match &callbacks.lock().unwrap().changed {
            Some(func) => {
              let _ = func.call1(py, (msg,));
            }
            None => {}
          }
          // let wrapper: WebsocketResult = serde_json::from_str(&msg).unwrap();
        }
        StatusMessage::Disconnected => {
          match &callbacks.lock().unwrap().disconnected {
            Some(func) => {
              let _ = func.call1(py, ("disconnected",));
            }
            None => {}
          }
        }
        StatusMessage::ConnectionShutdown => {
          match &callbacks.lock().unwrap().disconnected {
            Some(func) => {
              let _ = func.call1(py, ("Shutdown",));
              //let _ = func.call1(py, (ack_object,));
            }
            None => {}
          }
        }
        StatusMessage::Error(e) => {
          info!("status error: {:?}", e);
          match &callbacks.lock().unwrap().error {
            Some(func) => {
              let _ = func.call1(py, (e.to_string(),));
            }
            None => {}
          }
        }
      },

      Err(e) => {
        info!("error: {:?}", e);
        match &callbacks.lock().unwrap().error {
          Some(func) => {
            let _ = func.call1(py, (e.to_string(),));
          }
          None => {}
        }
      }
    });
    // info!("loop callback");
    sleep(std::time::Duration::from_millis(100))
  }
}
