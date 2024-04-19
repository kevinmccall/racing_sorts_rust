use std::sync::mpsc::Sender;

use crate::racer::{SortMessage, SortRunner};

use super::SortBase;

pub struct BubbleSorter<'a, T: PartialOrd> {
    sortbase: SortBase<'a, T>,
}

impl<'a, T: PartialOrd> BubbleSorter<'a, T> {
    pub fn new(data: Vec<T>, id: u8, sender: Sender<SortMessage<T>>) -> Self {
        BubbleSorter {
            sortbase: SortBase::new(data, id, sender),
        }
    }
}

impl<'a, T: PartialOrd> SortRunner<T> for BubbleSorter<'a, T> {
    fn sort(&mut self) {
        self.sortbase.notify();
        for i in 0..self.sortbase.data().len() {
            for j in i + 1..self.sortbase.data().len() {
                if self.sortbase.data()[i] > self.sortbase.data()[j] {
                    self.sortbase.swap(i, j);
                }
            }
        }
    }
}
