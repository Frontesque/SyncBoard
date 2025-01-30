use tungstenite::{ connect, Message };

pub fn start(server_address: String) {
    let connection = connect(&server_address);
    let connection_status = match connection {
        Ok(socket) => socket,
        Err(_) => return,
    };
    let (mut socket, _) = connection_status;
    println!("[SyncBoard Client] Connected to: {}", server_address);

    let message_handler = socket.read();
    let msg = match message_handler {
        Ok(msg) => msg,
        Err(_) => return,
    };
    let _ = socket.send(Message::Text("Hello!".into()));
    println!("* Server: {msg}");
}