use std::{
    sync::{mpsc::Sender, Arc, Condvar, Mutex, MutexGuard},
    thread,
};

use crate::racer::{SortMessage, SortRunner, SLEEP_DURATION};

pub struct QuickSorter<T: PartialOrd> {
    data: Arc<Mutex<Vec<T>>>,
    // TODO make this have a boolean in case of early wakeups
    condvar: Arc<Condvar>,
    id: u8,
}

impl<T: PartialOrd> QuickSorter<T> {
    pub fn new(data: Vec<T>, id: u8) -> Self {
        QuickSorter {
            data: Arc::new(Mutex::new(data)),
            condvar: Arc::new(Condvar::new()),
            id,
        }
    }

    fn quick_sort<'a>(
        &'a self,
        guard: MutexGuard<'a, Vec<T>>,
        start: usize,
        end: usize,
        sender: &Sender<SortMessage<T>>,
    ) -> MutexGuard<'a, Vec<T>> {
        let mut ret_guard = guard;
        if start < end {
            let (guard_ret, pivot) = self.partition(ret_guard, start, end, sender);
            ret_guard = guard_ret;
            if pivot != 0 {
                ret_guard = self.quick_sort(ret_guard, start, pivot - 1, sender);
            }
            ret_guard = self.quick_sort(ret_guard, pivot + 1, end, sender);
        }
        ret_guard
    }

    fn partition<'a>(
        &'a self,
        mut guard: MutexGuard<'a, Vec<T>>,
        l: usize,
        r: usize,
        sender: &Sender<SortMessage<T>>,
    ) -> (MutexGuard<'a, Vec<T>>, usize) {
        let mut i = l;
        let mut j = r + 1;

        loop {
            loop {
                i += 1;
                if !(i <= r && guard[i] <= guard[l]) {
                    break;
                }
            }
            loop {
                j -= 1;
                if !(guard[j] > guard[l]) {
                    break;
                }
            }
            if i >= j {
                break;
            }

            guard.swap(i, j);
            let message = SortMessage {
                id: self.id,
                name: "quick_sort",
                data: self.data.clone(),
                condvar: self.condvar.clone(),
            };
            sender.send(message).unwrap();
            guard = self.condvar.wait(guard).unwrap();
            thread::sleep(SLEEP_DURATION);
        }

        guard.swap(l, j);
        let message = SortMessage {
            id: self.id,
            name: "quick_sort",
            data: self.data.clone(),
            condvar: self.condvar.clone(),
        };
        sender.send(message).unwrap();
        guard = self.condvar.wait(guard).unwrap();
        thread::sleep(SLEEP_DURATION);

        (guard, j)
    }
}

impl<T: PartialOrd> SortRunner<T> for QuickSorter<T> {
    fn sort(&self, sender: Sender<SortMessage<T>>) {
        let mut data = self.data.lock().unwrap();
        let message = SortMessage {
            id: self.id,
            data: self.data.clone(),
            name: "quick_sort",
            condvar: self.condvar.clone(),
        };
        sender.send(message).unwrap();
        data = self.condvar.wait(data).unwrap();

        if data.len() == 0 {
            return;
        }
        let start = 0;
        let end = data.len() - 1;
        let _no = self.quick_sort(data, start, end, &sender);
    }
}
