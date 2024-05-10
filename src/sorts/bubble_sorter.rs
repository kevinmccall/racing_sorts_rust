use std::{
    sync::{mpsc::Sender, Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

use crate::racer::{SortMessage, SortRunner, SLEEP_DURATION};

pub struct BubbleSorter<T: PartialOrd> {
    data: Arc<Mutex<Vec<T>>>,
    // TODO make this have a boolean in case of early wakeups
    condvar: Arc<Condvar>,
    id: u8,
}

impl<T: PartialOrd> BubbleSorter<T> {
    pub fn new(data: Vec<T>, id: u8) -> Self {
        BubbleSorter {
            data: Arc::new(Mutex::new(data)),
            condvar: Arc::new(Condvar::new()),
            id,
        }
    }
}

impl<T: PartialOrd> SortRunner<T> for BubbleSorter<T> {
    fn sort(&self, sender: Sender<SortMessage<T>>) {
        let mut data = self.data.lock().unwrap();
        let message = SortMessage {
            id: self.id,
            data: self.data.clone(),
            condvar: self.condvar.clone(),
            name: "bubble_sort",
        };
        sender.send(message).unwrap();
        data = self.condvar.wait(data).unwrap();

        for i in 0..data.len() {
            for j in i + 1..data.len() {
                if data[i] > data[j] {
                    data.swap(i, j);
                    let message = SortMessage {
                        id: self.id,
                        data: self.data.clone(),
                        condvar: self.condvar.clone(),
                        name: "bubble_sort",
                    };
                    sender.send(message).unwrap();
                    data = self.condvar.wait(data).unwrap();
                    thread::sleep(SLEEP_DURATION);
                }
            }
        }
    }
}
