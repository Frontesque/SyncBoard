use arboard::Clipboard;

pub fn set(contents: String) -> bool {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(contents).unwrap();
    return true;
}

pub fn get() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    let contents_handler = clipboard.get_text();
    let contents = match contents_handler {
        Ok(clipboard) => clipboard,
        Err(cberr) => {
            println!("[SyncBoard Clipboard] Error: {}", cberr);
            "".to_string()
        }
    };
    return contents;
}