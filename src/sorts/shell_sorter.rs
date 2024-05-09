use std::sync::{mpsc::Sender, Arc, Condvar, Mutex};

use crate::racer::{SortMessage, SortRunner};

// Knuth's recommended increments
const H: [usize; 8] = [3280, 1093, 364, 121, 40, 13, 4, 1];
const SMAX: usize = 7;

pub struct ShellSorter<T: PartialOrd> {
    data: Arc<Mutex<Vec<T>>>,
    // TODO make this have a boolean in case of early wakeups
    condvar: Arc<Condvar>,
    id: u8,
}

impl<T: PartialOrd> ShellSorter<T> {
    pub fn new(data: Vec<T>, id: u8) -> Self {
        ShellSorter {
            data: Arc::new(Mutex::new(data)),
            condvar: Arc::new(Condvar::new()),
            id,
        }
    }
}

impl<T: PartialOrd> SortRunner<T> for ShellSorter<T> {
    fn sort(&self, sender: Sender<SortMessage<T>>) {
        let mut data = self.data.lock().unwrap();
        let message = SortMessage {
            id: self.id,
            data: self.data.clone(),
            condvar: self.condvar.clone(),
        };
        sender.send(message).unwrap();
        data = self.condvar.wait(data).unwrap();

        for s in 0..=SMAX {
            let step = H[s];
            for j in step..data.len() {
                let mut i = j - step;

                while data[i] > data[j] {
                    data.swap(i + step, i);
                    let message = SortMessage {
                        id: self.id,
                        data: self.data.clone(),
                        condvar: self.condvar.clone(),
                    };
                    sender.send(message).unwrap();
                    data = self.condvar.wait(data).unwrap();
                    if i >= step {
                        i -= step;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}
