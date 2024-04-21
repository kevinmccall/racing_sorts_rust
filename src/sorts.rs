use std::{cell::OnceCell, fmt::Debug, sync::{mpsc::Sender, Arc, Condvar, Mutex}};

use crate::racer::SortMessage;

pub mod bubble_sorter;
pub mod quick_sorter;

fn get_sort<T: PartialOrd>(
    sort_name: &str,
) -> Option<fn(&mut [T], &mut dyn FnMut(SortProgress))> {
    match sort_name {
        "bubble_sort" => Some(bubble_sorter::bubble_sort),
        "quick_sort" => Some(quick_sorter::quick_sort),
        _ => None,
    }
}

pub enum SortProgress {
    Start,
    InProgress,
    End
}

pub struct SortBase<T: PartialOrd + Debug> {
    id: u8,
    // TODO make this have a boolean in case of early wakeups
    data: Arc<Mutex<Vec<T>>>,
    condvar: Arc<Condvar>,
    sender: Sender<SortMessage<T>>,
}

impl<T: PartialOrd + Debug> SortBase<T> {
    pub fn new(data: Vec<T>, id: u8, sender: Sender<SortMessage<T>>) -> Self {
        let my_arc = Arc::new(Mutex::new(data));
        SortBase {
            data: my_arc,
            id,
            condvar: Arc::new(Condvar::new()),
            sender,
        }
    }

    pub fn sort(&self, sort_fn: fn(&mut [T], &mut dyn FnMut(SortProgress))) {
        let mut data = OnceCell::new();
        data.set(self.data.lock().unwrap()).unwrap();
        let mut snapshot = |progress: SortProgress| {
            let message = SortMessage {
                id: self.id,
                data: self.data.clone(),
                condvar: self.condvar.clone(),
                progress
            };
            self.sender.send(message).unwrap();
            // let mut lock = data.take().unwrap();
            // lock = self.condvar.wait(lock).unwrap();
            // data.set(lock);
        };
        sort_fn(data.get_mut().unwrap(), &mut snapshot);
    }
}
