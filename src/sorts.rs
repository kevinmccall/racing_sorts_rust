use crate::racer::SortRunner;

pub mod bubble_sorter;
pub mod insertion_sorter;
pub mod quick_sorter;
pub mod selection_sorter;
pub mod shell_sorter;

fn get_sorter<T: PartialOrd + 'static>(
    data: Vec<T>,
    sort_name: &str,
    id: u8,
) -> Option<Box<dyn SortRunner<T>>> {
    match sort_name {
        "bubble_sort" => Some(Box::new(bubble_sorter::BubbleSorter::new(data, id))),
        "quick_sort" => Some(Box::new(quick_sorter::QuickSorter::new(data, id))),
        "insertion_sort" => Some(Box::new(insertion_sorter::InsertionSorter::new(data, id))),
        "selection_sort" => Some(Box::new(selection_sorter::SelectionSorter::new(data, id))),
        "shell_sort" => Some(Box::new(shell_sorter::ShellSorter::new(data, id))),
        _ => None,
    }
}
