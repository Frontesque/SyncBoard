use arboard::Clipboard;
use std::thread::spawn;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::Sender;

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

pub fn watch(tx: Sender<String>) {
    spawn(move || {
        let mut last_clipboard: String = "".to_string();
        loop {
            let this_clipboard = get();
            if last_clipboard != this_clipboard {
                println!("[SyncBoard Watcher] Update: {}", this_clipboard);
                tx.send(this_clipboard.clone()).unwrap();
                last_clipboard = this_clipboard;
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });
}