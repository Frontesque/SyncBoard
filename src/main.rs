use std::env;
mod server;
mod client;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return eprintln!("Usage: {} <action>", args[0]);
    }
    
    match args[1].as_str() {
        "server" => {
            server::start(9055);
        },
        "client" => {
            client::start("ws://localhost:9055".to_string());
        },
        _  => { return eprintln!("Usage: {} <action>", args[0]); }
    }
}
