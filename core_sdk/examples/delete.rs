use core_sdk::sdk::TodoSdk;

fn main() {
  env_logger::init();
  let id = "6780d36b21963b05f0801f2b".to_string();
  let sdk = TodoSdk::new(None);
  sdk.delete(id).expect("delete a task failed");
}
