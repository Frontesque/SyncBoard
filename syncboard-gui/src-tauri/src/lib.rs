#[tauri::command]
fn something() -> String {
  return "Hello".to_string();
}

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![something])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
