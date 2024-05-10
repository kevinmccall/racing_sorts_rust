use std::{
    sync::{mpsc::Sender, Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

use crate::racer::{SortMessage, SortRunner, SLEEP_DURATION};

pub struct InsertionSorter<T: PartialOrd> {
    data: Arc<Mutex<Vec<T>>>,
    // TODO make this have a boolean in case of early wakeups
    condvar: Arc<Condvar>,
    id: u8,
}

impl<T: PartialOrd> InsertionSorter<T> {
    pub fn new(data: Vec<T>, id: u8) -> Self {
        InsertionSorter {
            data: Arc::new(Mutex::new(data)),
            condvar: Arc::new(Condvar::new()),
            id,
        }
    }
}

impl<T: PartialOrd> SortRunner<T> for InsertionSorter<T> {
    fn sort(&self, sender: Sender<SortMessage<T>>) {
        let mut data = self.data.lock().unwrap();
        let message = SortMessage {
            id: self.id,
            data: self.data.clone(),
            name: "insertion_sort",
            condvar: self.condvar.clone(),
        };
        sender.send(message).unwrap();
        data = self.condvar.wait(data).unwrap();

        for i in 1..data.len() {
            for j in (1..=i).rev() {
                if data[j - 1] > data[j] {
                    // I must use swap because I cannot move the elements out of
                    // my vector.
                    data.swap(j - 1, j);
                    let message = SortMessage {
                        id: self.id,
                        data: self.data.clone(),
                        name: "insertion_sort",
                        condvar: self.condvar.clone(),
                    };
                    sender.send(message).unwrap();
                    data = self.condvar.wait(data).unwrap();
                    thread::sleep(SLEEP_DURATION);
                } else {
                    break;
                }
            }
        }
    }
}
