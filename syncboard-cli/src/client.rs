use tungstenite::connect;
use std::sync::mpsc;
use crate::utils;
use crate::clipboard;

pub fn start(server_address: String) {
    //---   Connect to Server   ---//
    let connection = connect(&server_address);
    let connection_status = match connection {
        Ok(websocket) => websocket,
        Err(_) => return,
    };
    let (mut websocket, _) = connection_status;
    println!("[SyncBoard Client] Connected to: {}", server_address);
    let _ = websocket.send(utils::build_ws_message("new_client", env!("CARGO_PKG_VERSION")));

    //---   Watch Clipboard   ---//
    let (tx, rx) = mpsc::channel();
    clipboard::watch(tx);
    for received in rx {
        let _ = websocket.send(utils::build_ws_message("update_clipboard", received.clone().as_str()));
    }

    //---   Listen for messages   ---//
    loop {
        let message_handler = websocket.read();
        let msg = match message_handler {
            Ok(msg) => msg,
            Err(_) => return,
        };
        

        let [identifier, contents] = utils::deconstruct_ws_message(msg.to_string().as_str());
        match identifier.as_str() {
            "new_server" => {
                if contents == env!("CARGO_PKG_VERSION") {
                    println!("[SyncBoard Client] The client is running the same version as the server ({}). You're all set!", env!("CARGO_PKG_VERSION"));
                } else {
                    println!("[SyncBoard Client] The client is running a different version from the server. Client: {}, Server: {}. You may face issues.", env!("CARGO_PKG_VERSION"), contents);
                }
            },
            "update_clipboard" => {
                println!("[SyncBoard Client] Clipboard update: {}", contents);
                clipboard::set(contents);
            },
            _  => { println!("[SyncBoard Client] Recieved an unknown message from server: {}", msg) }
        }
    }
}