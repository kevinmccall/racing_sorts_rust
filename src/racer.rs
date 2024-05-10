use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::u8;

use crate::screen::ScreenManager;

use crate::sorts::get_sorter;

use crate::sorts::is_sort_valid;

pub const SLEEP_DURATION: Duration = Duration::from_millis(20);

/// I am borrowing some sorts from the internet: https://www.kirillvasiltsov.com/writing/sorting-algorithms-in-rust/

pub trait SortRunner<T: PartialOrd> {
    fn sort(&self, sender: Sender<SortMessage<T>>);
}

pub struct SortMessage<T> {
    pub id: u8,
    pub data: Arc<Mutex<Vec<T>>>,
    pub condvar: Arc<Condvar>,
    pub name: &'static str,
}

pub fn sort_manager(sorts: Vec<String>, data: Vec<u8>) -> Result<(), &'static str> {
    let manager = ScreenManager::init_screen();
    if sorts.len() > manager.get_num_rows() as usize {
        return Err("Too many sorts specified. All of them should be able to fit on screen");
    }

    manager.clear_screen();

    let mut id = 0;
    if !sorts
        .iter()
        .map(|sort_name| is_sort_valid(sort_name))
        .all(|x| x)
    {
        return Err("At least one of your sorts are invalid");
    }

    let (sender, receiver) = mpsc::channel();
    for sort_name in sorts {
        let sender = sender.clone();
        let data = data.clone();
        thread::spawn(move || {
            let sort_runner = get_sorter(data, &sort_name, id).unwrap();
            sort_runner.sort(sender);
        });
        id += 1;
    }
    drop(sender);

    while let Ok(message) = receiver.recv() {
        let data = message.data.lock().unwrap();
        let display = std::str::from_utf8(&data).unwrap_or("error");
        // let on_screen = format!("{}, {}: {:?}", message.id, message.name, display);
        let on_screen = format!("{}: {:?}", message.id, display);
        manager.string_at_pos(&on_screen, message.id as u32, 0);
        message.condvar.notify_one();
    }
    Ok(())
}
