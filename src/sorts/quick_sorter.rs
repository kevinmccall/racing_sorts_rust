use super::SortProgress;

pub fn quick_sort<T: PartialOrd>(array: &mut [T], snapshot: &mut dyn FnMut(SortProgress)) {
    let start = 0;
    let end = array.len() - 1;
    snapshot(SortProgress::Start);
    quick_sort_partition(array, start, end as isize, snapshot);
    snapshot(SortProgress::End);
}

fn quick_sort_partition<T: PartialOrd>(array: &mut [T], start: isize, end: isize, snapshot: &mut dyn FnMut(SortProgress)) {
    if start < end && end - start >= 1 {
    let pivot = partition(array, start as isize, end as isize, snapshot);
    quick_sort_partition(array, start, pivot - 1, snapshot);
    quick_sort_partition(array, pivot + 1, end, snapshot);
    }
}

fn partition<T: PartialOrd>(array: &mut [T], l: isize, h: isize, snapshot: &mut dyn FnMut(SortProgress)) -> isize {
    let mut i = l - 1; // Index of the smaller element

    for j in l..h {
        if array[j as usize] <= array[h as usize] {
            i = i + 1;
            array.swap(i as usize, j as usize);
            snapshot(SortProgress::InProgress);
        }
    }

    array.swap((i + 1) as usize, h as usize);
    snapshot(SortProgress::InProgress);
    i + 1
}