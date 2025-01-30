use arboard::Clipboard;

pub fn set_clipboard(contents: String) -> bool {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(contents).unwrap();
    return true;
}

pub fn get_clipboard() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    let contents = clipboard.get_text().unwrap();
    return contents;
}

// fn watch() {
//     let mut last_clipboard: String = "".to_string();
//     loop {
//         let this_clipboard = get_clipboard();
//         if last_clipboard != this_clipboard {
//             println!("Clipboard updated: {}", this_clipboard);
//             last_clipboard = this_clipboard;
//         }
//     }
// }