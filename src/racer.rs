use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::u8;

use crate::screen::ScreenManager;

/// I am borrowing some sorts from the internet: https://www.kirillvasiltsov.com/writing/sorting-algorithms-in-rust/

pub trait SortRunner<T: PartialOrd> {
    fn sort(&mut self);
}

pub struct SortMessage<T> {
    pub id: u8,
    pub data: Arc<Mutex<Vec<T>>>,
    pub condvar: Arc<Condvar>,
}

// pub fn sort_manager(data: Vec<u8>) {
//     let manager = ScreenManager::init_screen();
//     manager.clear_screen();
//     let (sender, receiver) = mpsc::channel();

//     let num_sorters = 1;

//     for i in 0..num_sorters {
//         let sender = sender.clone();
//         let data = data.clone();
//         thread::spawn(move || {
//             let mut sort_runner = QuickSorter::new(data, i, sender);
//             sort_runner.sort();
//         });
//     }
//     drop(sender);

//     while let Ok(message) = receiver.recv() {
//         let data = message.data.lock().unwrap();
//         let display = std::str::from_utf8(&data).unwrap_or("error");
//         let on_screen = format!("{}: {:?}", message.id, display);
//         thread::sleep(Duration::from_millis(1000));
//         // manager.string_at_pos(&on_screen, message.id as u32, 0);
//         println!("{}", on_screen);
//         message.condvar.notify_one();
//     }
// }
