use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::u8;

use crate::screen::ScreenManager;
use crate::sorts::bubble_sorter::BubbleSorter;
use crate::sorts::insertion_sorter::InsertionSorter;
use crate::sorts::quick_sorter::QuickSorter;
use crate::sorts::selection_sorter::SelectionSorter;
use crate::sorts::shell_sorter::ShellSorter;

/// I am borrowing some sorts from the internet: https://www.kirillvasiltsov.com/writing/sorting-algorithms-in-rust/

pub trait SortRunner<T: PartialOrd> {
    fn sort(&self, sender: Sender<SortMessage<T>>);
}

pub struct SortMessage<T> {
    pub id: u8,
    pub data: Arc<Mutex<Vec<T>>>,
    pub condvar: Arc<Condvar>,
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
            let sort_runner = SelectionSorter::new(data, i);
            sort_runner.sort(sender);
        });
    }
    drop(sender);

    let mut counter = 0u32;

    while let Ok(message) = receiver.recv() {
        let data = message.data.lock().unwrap();
        let display = std::str::from_utf8(&data).unwrap_or("error");
        let on_screen = format!("{}: {:?}", message.id, display);
        thread::sleep(Duration::from_millis(2));
        manager.string_at_pos(&on_screen, message.id as u32, 0);
        // println!("{}", on_screen);
        counter += 1;
        message.condvar.notify_one();
    }
    eprintln!("\n{} swaps conducted", counter);
}
