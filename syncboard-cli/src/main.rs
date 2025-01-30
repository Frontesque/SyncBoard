mod server;
mod client;
pub mod clipboard;
pub mod utils;
use std::thread;
use std::time::Duration;

fn help() {
    println!("
###   SyncBoard Help   ###
1. syncboard server
    - Starts SyncBoard in server mode

2. syncboard client <ip_address>
    - Starts SyncBoard in client mode and attempts to connect to a SyncBoard server running on the given <ip_address>

3. syncboard test
    - Tests SyncBoard capabilities on your system

4. syncboard help
    - Shows this message
");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { // Runs if the program is given with no additional args
        println!("[Syncboard] No arguments provided. Showing help.");
        help();
        println!("[SyncBoard] The program will exit automatically in 10 seconds.");
        thread::sleep(Duration::from_millis(10000));
        return;
    }
    
    println!("[SyncBoard] Welcome to SyncBoard v{}!", env!("CARGO_PKG_VERSION"));
    match args[1].as_str() {
        "server" => {
            println!("[SyncBoard] Starting server...");
            server::start(9055);
        },
        "client" => {
            println!("[SyncBoard] Starting client...");
            if args.len() < 3 { return help(); }
            let server_address = format!("ws://{}:9055",args[2]);
            client::start(server_address.to_string());
        },
        "test" => {
            println!("[SyncBoard] Testing SyncBoard capabilities on your system!");
            println!("[SyncBoard Test] Your current clipboard is: \"{}\"", clipboard::get());
            let test_string: String = format!("You're running SyncBoard v{}!", env!("CARGO_PKG_VERSION"));
            clipboard::set(test_string.clone());
            println!("[SyncBoard Test] Your clipboard should now contain: \"{}\".", test_string);
            println!("[SyncBoard Test] Try pasting somewhere!");
        }
        _  => { return help(); }
    }
    println!("[SyncBoard] Exiting.");
}
