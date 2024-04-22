use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::Thread;
use std::time::Duration;
use std::u8;

use crate::screen::ScreenManager;
use crate::sorts::bubble_sorter::bubble_sort;
use crate::sorts::quick_sorter::quick_sort;
use crate::sorts::SortBase;
use crate::sorts::SortProgress;

/// I am borrowing some sorts from the internet: https://www.kirillvasiltsov.com/writing/sorting-algorithms-in-rust/

pub struct SortMessage<T> {
    pub id: u8,
    pub data: Arc<Mutex<Vec<T>>>,
    pub thread: Thread,
    pub progress: SortProgress
}

pub fn sort_manager(data: Vec<u8>) {
    let manager = ScreenManager::init_screen();
    manager.clear_screen();
    let (sender, receiver) = mpsc::channel();

    let num_sorters = 1;

    for i in 0..num_sorters {
        let sender = sender.clone();
        let data = data.clone();
        thread::spawn(move || {
            let sort_runner = SortBase::new(data, i, sender);
            sort_runner.sort(bubble_sort);
        });
    }
    drop(sender);

    while let Ok(message) = receiver.recv() {
        println!("I am from racer!! message received!!");
        let data = message.data.lock().unwrap();
        let display = std::str::from_utf8(&data).unwrap_or("error");
        let on_screen = format!("{}: {:?}", message.id, display);
        thread::sleep(Duration::from_millis(1000));
        println!("about to... unpark!!??!");
        message.thread.unpark();
        println!("Done UNPARKING!!");
        // manager.string_at_pos(&on_screen, message.id as u32, 0);
        println!("{}", on_screen);
        
    }
}
