use crate::racer::SortRunner;

pub mod bubble_sorter;

pub const KNOWN_SORTS: [&str; 5] = [
    "bubble_sort",
    "quick_sort",
    "insertion_sort",
    "selection_sort",
    "shell_sort",
];

// pub fn get_sorter<T: PartialOrd + 'static>(
//     data: Vec<T>,
//     sort_name: &str,
//     id: u8,
// ) -> Option<Box<dyn SortRunner<T>>> {
//     match sort_name {
//         "bubble_sort" => Some(Box::new(bubble_sorter::BubbleSorter::new(data, id))),
//         _ => None,
//     }
// }

pub fn is_sort_valid(sort_name: &str) -> bool {
    return KNOWN_SORTS.contains(&sort_name);
}
