pub fn bubble_sort<T: PartialOrd + Copy>(data: &mut [T]) {
    for i in 0..data.len() {
        for j in i+1..data.len() {
            if data[i] > data[j] {
                let temp = data[i];
                data[i] = data[j];
                data[j] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorts::tests::test_sort;
    use super::*;

    #[test]
    fn test_bubble() {
        test_sort::<u32>(1000, bubble_sort);
    }
}