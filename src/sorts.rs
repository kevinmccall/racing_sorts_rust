use crate::racer::SortRunner;

pub mod bubble_sorter;
pub mod quick_sorter;

fn get_sorter<T: PartialOrd + 'static>(
    data: Vec<T>,
    sort_name: &str,
    id: u8,
) -> Option<Box<dyn SortRunner<T>>> {
    match sort_name {
        "bubble_sort" => Some(Box::new(bubble_sorter::BubbleSorter::new(data, id))),
        "quick_sort" => Some(Box::new(quick_sorter::QuickSorter::new(data, id))),
        _ => None,
    }
}
