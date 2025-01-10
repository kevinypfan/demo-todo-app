use core_sdk::sdk::TodoSdk;

fn main() {
  env_logger::init();
  let sdk = TodoSdk::new(None);
  let result = sdk.list().expect("get list of task failed");
  println!("{:?}", result);
}
