use core_sdk::sdk::TodoSdk;

fn main() {
  env_logger::init();
  let id = "67816506fffcf979782d993f".to_string();
  let sdk = TodoSdk::new(None);
  let result = sdk
    .edit(id, "core sdk edit a task".to_string(), Some(true))
    .expect("get a task failed");
  println!("{:?}", result);
}
