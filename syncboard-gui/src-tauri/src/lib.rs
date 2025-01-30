//---   CLI Imports   ---//
#[path = "../../../syncboard-cli/src/clipboard.rs"] pub mod clipboard;
#[path = "../../../syncboard-cli/src/utils.rs"]     pub mod utils;
#[path = "../../../syncboard-cli/src/client.rs"]    pub mod syncboard_client;
#[path = "../../../syncboard-cli/src/server.rs"]    pub mod syncboard_server;

//---   Standard Imports   ---//
use std::thread::spawn;

//---   Tauri Functions   ---//
#[tauri::command]
fn start_syncboard_client(address: String) -> bool {
  let server_address = format!("ws://{}:9055",address);
  spawn(move || {
    syncboard_client::start(server_address);
  });
  return true;
}

#[tauri::command]
fn start_syncboard_server() -> bool {
  spawn(move || {
    syncboard_server::start(9055);
  });
  return true;
}


#[tauri::command]
fn syncboard_test() -> String {
  let old_clipboard = clipboard::get();
  let test_string: String = format!("You're running SyncBoard v{}!", env!("CARGO_PKG_VERSION"));
  clipboard::set(test_string);
  return old_clipboard;
}

#[tauri::command]
fn get_version() -> String {
  return env!("CARGO_PKG_VERSION").to_string();
}

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_version, syncboard_test, start_syncboard_server, start_syncboard_client])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
