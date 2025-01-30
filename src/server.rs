use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

/// A WebSocket echo server
pub fn start(port: u32) {
    let bind: String = format!("0.0.0.0:{}", port);
    println!("[SyncBoard Server] Listening on: {}", bind);
    let server = TcpListener::bind(bind).unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    println!("[SyncBoard Server] Message from client: {}", msg);
                    websocket.send(msg).unwrap();
                }
            }
        });
    }
}