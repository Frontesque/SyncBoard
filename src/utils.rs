use serde_json::{ from_str, Value, json};
use tungstenite::Message;

pub fn build_ws_message(identifier: &str, contents: &str) -> Message  {
    let output = json!({
        "identifier" : identifier,
        "contents"   : contents
    });
    return Message::text(output.to_string());
}

pub fn deconstruct_ws_message(ws_message: &str) -> [String; 2] {
    let raw_message: Value = from_str(ws_message).unwrap();
    let output: [String; 2] = [
        raw_message["identifier"].as_str().unwrap_or("").to_owned(), 
        raw_message["contents"].as_str().unwrap_or("").to_owned()
    ];
    return output;
}