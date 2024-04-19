use std::sync::{mpsc::Sender, Arc, Condvar, Mutex, MutexGuard};

use crate::racer::{SortMessage, SortRunner};

pub mod bubble_sorter;
pub mod quick_sorter;

// fn get_sorter<T: PartialOrd + 'static>(
//     data: Vec<T>,
//     sort_name: &str,
//     id: u8,
// ) -> Option<Box<dyn SortRunner<T>>> {
//     match sort_name {
//         "bubble_sort" => Some(Box::new(bubble_sorter::BubbleSorter::new(data, id))),
//         "quick_sort" => Some(Box::new(quick_sorter::QuickSorter::new(data, id))),
//         _ => None,
//     }
// }

pub struct SortBase<'a, T: PartialOrd> {
    id: u8,
    guard: MutexGuard<'a, Vec<T>>,
    // TODO make this have a boolean in case of early wakeups
    data: Arc<Mutex<Vec<T>>>,
    condvar: Arc<Condvar>,
    sender: Sender<SortMessage<T>>,
}

impl<'a, T: PartialOrd> SortBase<'a, T> {
    pub fn new(data: Vec<T>, id: u8, sender: Sender<SortMessage<T>>) -> Self {
        let my_arc = Arc::new(Mutex::new(data));
        let guard = my_arc.lock().unwrap();
        SortBase {
            data: my_arc,
            guard,
            id,
            condvar: Arc::new(Condvar::new()),
            sender,
        }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.guard.swap(i, j);
        self.notify();
    }

    pub fn notify(&mut self) {
        let message = SortMessage {
            id: self.id,
            data: self.data.clone(),
            condvar: self.condvar.clone(),
        };
        self.sender.send(message).unwrap();
        self.guard = self.condvar.wait(self.guard).unwrap();
    }

    pub fn data(&self) -> &Vec<T> {
        &*self.guard
    }
}
