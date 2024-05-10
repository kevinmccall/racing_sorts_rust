use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;
use std::u8;

use crate::screen::ScreenManager;

use crate::sorts::bubble_sorter;
// use crate::sorts::get_sorter;

use crate::sorts::is_sort_valid;

pub const SLEEP_DURATION: Duration = Duration::from_millis(20);

pub trait SortRunner<T: PartialOrd> {
    fn sort(&self, sender: Sender<SortMessage<T>>);
}

pub struct SortMessage<T> {
    pub id: u8,
    pub data: Arc<OnceLock<Vec<T>>>,
    pub condvar: Arc<Condvar>,
}

pub struct SortBase<T: PartialOrd> {
    condvar: Arc<Condvar>,
    sender: Sender<SortMessage<T>>,
    id: u8,
}

impl<T: PartialOrd> SortBase<T> {
    pub fn new(sender: Sender<SortMessage<T>>, id: u8) -> Self {
        SortBase {
            condvar: Arc::new(Condvar::new()),
            sender,
            id,
        }
    }

    pub fn send_update(&self, data: Vec<T>) -> Vec<T> {
        let mut shared = Arc::new({
            let lock = OnceLock::new();
            lock.set(data);
            lock
        });
        let message = SortMessage {
            id: self.id,
            data: shared.clone(),
            condvar: self.condvar.clone(),
        };
        self.sender.send(message).unwrap();
        thread::sleep(SLEEP_DURATION);
        shared.take().unwrap()
    }
}

pub fn test() {
    let manager = ScreenManager::init_screen();
    manager.clear_screen();
    let mut id = 0;
    let data = "asngioergnalerkgjreoingoiorjgwil4t".bytes().collect();
    let (sender, receiver) = mpsc::channel();
    {
        thread::spawn(move || {
            let manager = SortBase::new(sender, id);
            bubble_sorter::sort(data, manager);
        });
    }

    while let Ok(message) = receiver.recv() {
        let data = message.data.get().unwrap();
        let display = std::str::from_utf8(data).unwrap_or("error");
        // let on_screen = format!("{}, {}: {:?}", message.id, message.name, display);
        let on_screen = format!("{}: {:?}", message.id, display);
        manager.string_at_pos(&on_screen, message.id as u32, 0);
        message.condvar.notify_one();
    }
}
