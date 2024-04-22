use std::{borrow::BorrowMut, cell::OnceCell, fmt::Debug, sync::{mpsc::Sender, Arc, Mutex}, thread};

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
    sender: Sender<SortMessage<T>>,
}

impl<T: PartialOrd + Debug> SortBase<T> {
    pub fn new(data: Vec<T>, id: u8, sender: Sender<SortMessage<T>>) -> Self {
        let my_arc = Arc::new(Mutex::new(data));
        SortBase {
            data: my_arc,
            id,
            sender,
        }
    }

    pub fn sort(&self, sort_fn: fn(&mut [T], &mut dyn FnMut(SortProgress))) {
        let mut data = self.data.lock().unwrap();
        let mut snapshot = |progress: SortProgress| {
            println!("I have been called upon");
            let message = SortMessage {
                id: self.id,
                data: self.data.clone(),
                thread: thread::current(),
                progress
            };
            self.sender.send(message).unwrap();
            println!("I have successfully sent a message");
            drop(data);
            thread::park();
            println!("I arise from my slumber");
        };
        sort_fn(&mut data, &mut snapshot);
    }
}
