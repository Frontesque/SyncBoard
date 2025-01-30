use tungstenite::connect;
use std::thread::spawn;
use std::thread;
use std::time::Duration;
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
    spawn(move || {
        let mut last_clipboard: String = "".to_string();
        loop {
            let this_clipboard = clipboard::get();
            if last_clipboard != this_clipboard {
                println!("[SyncBoard Watcher] Update: {}", this_clipboard);
                tx.send(this_clipboard.clone()).unwrap();
                last_clipboard = this_clipboard;
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });

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
            _  => { println!("[SyncBoard Client] Recieved an unknown message from server: {}", msg) }
        }
    }
}