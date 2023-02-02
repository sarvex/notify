use std::{io::Write, path::Path, time::Duration};

use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;

/// Example f
/// or debouncer
fn main() {
    // emit some events by changing a file
    std::thread::spawn(|| {
        let path = Path::new("test.txt");
        let _ = std::fs::remove_file(&path);
        println!("running 250ms events");
        for _ in 0..20 {
            std::fs::write(&path, b"Lorem ipsum").unwrap();
            std::thread::sleep(Duration::from_millis(250));
        }
        println!("waiting 20s");
        std::thread::sleep(Duration::from_millis(20000));
        println!("running 3s events");
        for _ in 0..20 {
            std::fs::write(&path, b"Lorem ipsum").unwrap();
            std::thread::sleep(Duration::from_millis(3000));
        }
    });

    // setup debouncer
    let (tx, rx) = std::sync::mpsc::channel();

    // No specific tickrate, max debounce time 2 seconds
    let mut debouncer = new_debouncer(Duration::from_secs(1), tx).unwrap();

    debouncer
        .watcher()
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();

    // print all events, non returning
    for events in rx {
        for e in events {
            println!("{:?}", e);
        }
    }
}
