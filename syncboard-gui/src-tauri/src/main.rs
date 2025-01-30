// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "../../../syncboard-cli/src/clipboard.rs"] pub mod clipboard;
#[path = "../../../syncboard-cli/src/utils.rs"]     pub mod utils;
#[path = "../../../syncboard-cli/src/client.rs"]    pub mod syncboard_client;
#[path = "../../../syncboard-cli/src/server.rs"]    pub mod syncboard_server;

fn main() {
  app_lib::run();
}
