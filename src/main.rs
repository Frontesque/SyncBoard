mod server;
mod client;
pub mod clipboard;
pub mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return eprintln!("Usage: {} <action>", args[0]);
    }
    
    println!("[SyncBoard] Welcome to SyncBoard v{}!", env!("CARGO_PKG_VERSION"));
    match args[1].as_str() {
        "server" => {
            println!("[SyncBoard] Starting server...");
            server::start(9055);
        },
        "client" => {
            println!("[SyncBoard] Starting client...");
            client::start("ws://localhost:9055".to_string());
        },
        "test" => {
            println!("[SyncBoard] Testing SyncBoard capabilities on your system!");
            println!("[SyncBoard Test] Your current clipboard is: \"{}\"", clipboard::get_clipboard());
            let test_string: String = format!("You're running SyncBoard v{}!", env!("CARGO_PKG_VERSION"));
            clipboard::set_clipboard(test_string.clone());
            println!("[SyncBoard Test] Your clipboard should now contain: \"{}\".", test_string);
            println!("[SyncBoard Test] Try pasting somewhere!");
        }
        _  => { return eprintln!("Usage: {} <action>", args[0]); }
    }
    println!("[SyncBoard] Exiting.");
}
