use std::{
    sync::{mpsc::Sender, Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

use crate::racer::{SortMessage, SortRunner, SLEEP_DURATION};

pub struct SelectionSorter<T: PartialOrd> {
    data: Arc<Mutex<Vec<T>>>,
    // TODO make this have a boolean in case of early wakeups
    condvar: Arc<Condvar>,
    id: u8,
}

impl<T: PartialOrd> SelectionSorter<T> {
    pub fn new(data: Vec<T>, id: u8) -> Self {
        SelectionSorter {
            data: Arc::new(Mutex::new(data)),
            condvar: Arc::new(Condvar::new()),
            id,
        }
    }
}

impl<T: PartialOrd> SortRunner<T> for SelectionSorter<T> {
    fn sort(&self, sender: Sender<SortMessage<T>>) {
        let mut data = self.data.lock().unwrap();
        let message = SortMessage {
            id: self.id,
            name: "selection_sort",
            data: self.data.clone(),
            condvar: self.condvar.clone(),
        };
        sender.send(message).unwrap();
        data = self.condvar.wait(data).unwrap();

        for i in 0..data.len() - 1 {
            let mut k = i;
            for j in i + 1..data.len() {
                if data[k] > data[j] {
                    k = j;
                }
            }
            data.swap(i, k);
            let message = SortMessage {
                id: self.id,
                data: self.data.clone(),
                name: "selection_sort",
                condvar: self.condvar.clone(),
            };
            sender.send(message).unwrap();
            data = self.condvar.wait(data).unwrap();
            thread::sleep(SLEEP_DURATION);
        }
    }
}
