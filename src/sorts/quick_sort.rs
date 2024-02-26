pub fn quick_sort<T: PartialOrd + Copy>(data: &mut [T]) {
    if data.len() < 2 {
        return;
    }
    let index = data.len() / 2;
    let pivot = data[index];
    let mut left = 0;
    let mut right = data.len() - 1;
    (data[index], data[0]) = (data[0], data[index]);
    while left < right {
        while data[left] < pivot {
            left += 1;
        }
        while data[right] > pivot {
            right -= 1;
        }
        if left < right {
            (data[left], data[right]) = (data[right], data[left])
        }
    }
    (data[right], data[0]) = (data[right], data[0]);
    quick_sort(&mut data[.. right]);
    quick_sort(&mut data[right + 1 ..]);
}

#[cfg(test)]
mod tests {
    use crate::sorts::tests::test_sort;
    use super::*;

    #[test]
    fn test_quick_sort() {
        test_sort(1000, quick_sort);
    }
}