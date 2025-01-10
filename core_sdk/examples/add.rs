use core_sdk::sdk::TodoSdk;

fn main() {
  env_logger::init();
  let sdk = TodoSdk::new(None);
  let result = sdk
    .add("core sdk add a task".to_string(), Some(false))
    .expect("add a task to todo list failed");

  println!("{:?}", result);
}
