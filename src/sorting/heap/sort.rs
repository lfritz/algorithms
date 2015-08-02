//! Algorithm 2.7 Heapsort

use super::sink;

/// Sort `v` using heapsort.
pub fn sort<T: Ord>(v: &mut Vec<T>) {
    let n = v.len();
    for k in (0..n/2).rev() {
        sink(v, k, n);
    }
    for k in (1..n).rev() {
        v.swap(0, k);
        sink(v, 0, k);
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    use super::super::super::tests::test_sort;

    #[test]
    fn sort_works() {
        test_sort(sort);
    }
}

