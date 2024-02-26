pub fn merge_sort<T: PartialOrd + Copy>(data: & [T]) -> Vec<T> {
    if data.len() == 1 {
        return data.to_vec();
    }
    let mid = data.len() / 2;
    let first = merge_sort(& data[.. mid]);
    let second = merge_sort(& data[mid .. ]);

    let mut out = Vec::with_capacity(data.len());

    let mut i = 0;
    let mut j = 0;
    while i < first.len() && j < second.len() {
        if first[i] < second[j] {
            out.push(first[i]);
            i += 1;
        } else {
            out.push(second[j]);
            j += 1;
        }
    }
    while i < first.len() {
        out.push(first[i]);
        i += 1;
    }
    while j < second.len() {
        out.push(second[j]);
        j += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::sorts::tests::rec_test_sort;
    use super::*;

    #[test]
    fn test_merge_better() {
        rec_test_sort(1000, merge_sort);
    }
}