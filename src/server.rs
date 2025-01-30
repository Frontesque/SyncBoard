use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;
use crate::utils;
use crate::clipboard;

pub fn start(port: u32) {
    let bind: String = format!("0.0.0.0:{}", port);
    println!("[SyncBoard Server] Listening on: {}", bind);
    let server = TcpListener::bind(bind).unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();
                if msg.is_text() {
                    // println!("[SyncBoard Server Debug] Message from client: {}", msg);



                    let [identifier, contents] = utils::deconstruct_ws_message(msg.to_string().as_str());
                    match identifier.as_str() {
                        "new_client" => {
                            println!("[SyncBoard Server] Client connected!");
                            websocket.send(utils::build_ws_message("new_server", env!("CARGO_PKG_VERSION"))).unwrap();
                            if contents == env!("CARGO_PKG_VERSION") {
                                println!("[SyncBoard Server] The client is running the same version as the server ({}). You're all set!", env!("CARGO_PKG_VERSION"));
                            } else {
                                println!("[SyncBoard Server] The client is running a different version from the server. Client: {}, Server: {}. You may face issues.", contents, env!("CARGO_PKG_VERSION"));
                            }
                        },
                        "update_clipboard" => {
                            println!("[SyncBoard Server] Clipboard update: {}", contents);
                            clipboard::set(contents);
                        },
                        _  => { println!("[SyncBoard Server] Recieved an unknown message from client: {}", msg) }
                    }



                }
            }
        });
    }
}