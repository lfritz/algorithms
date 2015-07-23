//! Algorithm 2.1: Selection sort.

/// Sort `v` using selection sort.
pub fn sort<T: Ord>(v: &mut Vec<T>) {
    let n = v.len();
    for i in 0..n {
        let mut min = i;
        for j in i+1..n {
            if v[j] < v[min] {
                min = j;
            }
        }
        v.swap(i, min);
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    use super::super::tests::test_sort;

    #[test]
    fn sort_works() {
        test_sort(sort);
    }
}
