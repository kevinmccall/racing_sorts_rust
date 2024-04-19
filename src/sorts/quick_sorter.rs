use std::sync::mpsc::Sender;

use crate::racer::{SortMessage, SortRunner};

use super::SortBase;

pub struct QuickSorter<'a, T: PartialOrd> {
    sb: SortBase<'a, T>,
}

impl<'a, T: PartialOrd> QuickSorter<'a, T> {
    pub fn new(data: Vec<T>, id: u8, sender: Sender<SortMessage<T>>) -> Self {
        QuickSorter {
            sb: SortBase::new(data, id, sender),
        }
    }

    fn quick_sort_partition(&mut self, start: isize, end: isize) {
        if start < end && end - start >= 1 {
            let pivot = self.partition(start as isize, end as isize);
            self.quick_sort_partition(start, pivot - 1);
            self.quick_sort_partition(pivot + 1, end);
        }
    }

    fn partition(&mut self, l: isize, h: isize) -> isize {
        let mut i = l - 1; // Index of the smaller element
        for j in l..h {
            if self.sb.data()[j as usize] <= self.sb.data()[h as usize] {
                i = i + 1;
                // println!("1swap {} and {}", i, j);
                self.sb.swap(i as usize, j as usize);
            }
        }
        // println!("2swap {} and {}", i + 1, h);
        self.sb.swap((i + 1) as usize, h as usize);

        i + 1
    }
}

impl<'a, T: PartialOrd> SortRunner<T> for QuickSorter<'a, T> {
    fn sort(&mut self) {
        self.sb.notify();

        let start = 0;
        let end = self.sb.data().len() - 1;
        self.quick_sort_partition(start, end as isize);
    }
}
