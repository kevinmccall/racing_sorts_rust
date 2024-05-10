use std::{
    sync::{mpsc::Sender, Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

use crate::racer::{SortBase, SortMessage, SortRunner, SLEEP_DURATION};

pub fn sort<T: PartialOrd>(mut data: Vec<T>, manager: SortBase<T>) -> Vec<T> {
    data = manager.send_update(data);
    for i in 0..data.len() {
        for j in i + 1..data.len() {
            if data[i] > data[j] {
                data.swap(i, j);
                data = manager.send_update(data);
            }
        }
    }

    data
}
