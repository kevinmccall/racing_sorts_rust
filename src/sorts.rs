use std::{cell::{OnceCell, RefCell}, sync::{mpsc::Sender, Arc, Condvar, Mutex, MutexGuard}};

use crate::racer::{SortMessage, SortRunner};

// pub mod bubble_sorter;
// pub mod quick_sorter;

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
    guard: OnceCell<MutexGuard<'a, Vec<T>>>,
    // TODO make this have a boolean in case of early wakeups
    data: &'a Arc<Mutex<Vec<T>>>,
    condvar: Arc<Condvar>,
    sender: Sender<SortMessage<T>>,
}

impl<'a, T: PartialOrd> SortBase<'a, T> {
    pub fn new(data: &'a Arc<Mutex<Vec<T>>>, id: u8, sender: Sender<SortMessage<T>>) -> Self {
        SortBase {
            data,
            guard: OnceCell::new(),
            id,
            condvar: Arc::new(Condvar::new()),
            sender,
        }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.guard.get_mut().unwrap().swap(i, j);
        self.notify();
    }

    pub fn notify(&mut self) {
        let message = SortMessage {
            id: self.id,
            data: self.data.clone(),
            condvar: self.condvar.clone(),
        };
        self.sender.send(message).unwrap();
        let mut lock = self.guard.take().unwrap();
        lock = self.condvar.wait(lock).unwrap();
        self.guard.set(lock);
    }

    pub fn data(&self) -> &Vec<T> {
        self.guard.get_or_init(|| {
            let guard = self.data.lock().unwrap();
            guard
        })
    }
}
