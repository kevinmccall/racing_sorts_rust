use std::sync::{mpsc::Sender, Arc, Condvar, Mutex};

use crate::racer::{SortMessage, SortRunner};

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
            condvar: self.condvar.clone(),
        };
        sender.send(message).unwrap();
        data = self.condvar.wait(data).unwrap();

        for i in 1..data.len() {
            for j in (2..=i).rev() {
                if data[j - 1] > data[j] {
                    // I must use swap because I cannot move the elements out of
                    // my vector.
                    data.swap(j - 1, j);

                    let message = SortMessage {
                        id: self.id,
                        data: self.data.clone(),
                        condvar: self.condvar.clone(),
                    };
                    sender.send(message).unwrap();
                    data = self.condvar.wait(data).unwrap();
                } else {
                    break;
                }
            }
        }
    }
}
