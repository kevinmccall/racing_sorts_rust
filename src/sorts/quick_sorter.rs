use std::sync::{mpsc::Sender, Arc, Condvar, Mutex, MutexGuard};

use crate::racer::{SortMessage, SortRunner};

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

    fn quick_sort_partition<'a>(
        &'a self,
        guard: MutexGuard<'a, Vec<T>>,
        start: isize,
        end: isize,
        sender: &Sender<SortMessage<T>>,
    ) -> MutexGuard<'a, Vec<T>> {
        let mut ret_guard = guard;
        if start < end && end - start >= 1 {
            let (guard_ret, pivot) =
                self.partition(ret_guard, start as isize, end as isize, sender);
            ret_guard = self.quick_sort_partition(guard_ret, start, pivot - 1, sender);
            ret_guard = self.quick_sort_partition(ret_guard, pivot + 1, end, sender);
        }
        ret_guard
    }

    fn partition<'a>(
        &'a self,
        mut guard: MutexGuard<'a, Vec<T>>,
        l: isize,
        h: isize,
        sender: &Sender<SortMessage<T>>,
    ) -> (MutexGuard<'a, Vec<T>>, isize) {
        let mut i = l - 1; // Index of the smaller element
        for j in l..h {
            if guard[j as usize] <= guard[h as usize] {
                i = i + 1;
                // println!("1swap {} and {}", i, j);
                guard.swap(i as usize, j as usize);
                let message = SortMessage {
                    id: self.id,
                    data: self.data.clone(),
                    condvar: self.condvar.clone(),
                };
                sender.send(message).unwrap();
                guard = self.condvar.wait(guard).unwrap();
            }
        }
        // println!("2swap {} and {}", i + 1, h);
        guard.swap((i + 1) as usize, h as usize);
        let message = SortMessage {
            id: self.id,
            data: self.data.clone(),
            condvar: self.condvar.clone(),
        };
        sender.send(message).unwrap();
        guard = self.condvar.wait(guard).unwrap();

        (guard, i + 1)
    }
}

impl<T: PartialOrd> SortRunner<T> for QuickSorter<T> {
    fn sort(&self, sender: Sender<SortMessage<T>>) {
        let mut data = self.data.lock().unwrap();
        let message = SortMessage {
            id: self.id,
            data: self.data.clone(),
            condvar: self.condvar.clone(),
        };
        sender.send(message).unwrap();
        data = self.condvar.wait(data).unwrap();

        let start = 0;
        let end = data.len() - 1;
        let _no = self.quick_sort_partition(data, start, end as isize, &sender);
    }
}
