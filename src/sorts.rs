mod merge_sort;
mod bubble_sort;

pub use merge_sort::merge_sort;
pub use bubble_sort::bubble_sort;

#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    pub fn rec_test_sort<T: PartialOrd + Copy>(n: u32, sort_fn: fn(& [u32]) -> Vec<u32>) {
        let nums: Vec<u32> = (1u32..n).collect();
        let mut shuffled = nums.clone();
        let mut rng = thread_rng();
        shuffled.shuffle(&mut rng);
        let shuffled = sort_fn(&shuffled);
        assert_eq!(nums, shuffled, "sort failed");
    }

    pub fn test_sort<T: PartialOrd + Copy>(n: u32, sort_fn: fn(&mut [u32])) {
        let nums: Vec<u32> = (1u32..n).collect();
        let mut shuffled = nums.clone();
        let mut rng = thread_rng();
        shuffled.shuffle(&mut rng);
        sort_fn(&mut shuffled);
        assert_eq!(nums, shuffled, "sort failed");
    }
}