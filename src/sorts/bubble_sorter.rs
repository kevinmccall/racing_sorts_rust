use super::SortProgress;

pub fn bubble_sort<T: PartialOrd>(data: &mut [T], snapshot: &mut dyn FnMut(SortProgress)) {
    snapshot(SortProgress::Start);
    for i in 0..data.len() {
        for j in i + 1..data.len() {
            if data[i] > data[j] {
                data.swap(i, j);
                snapshot(SortProgress::InProgress);
            }
        }
    }
    snapshot(SortProgress::End);
}
