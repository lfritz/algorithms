//! Algorithm 2.5 Bottom-up merge sort.

use std::cmp;

/// Sort `v` using a bottom-up merge sort.
pub fn sort<T: Ord + Copy>(v: &mut Vec<T>) {
    let n = v.len();
    let mut aux = v.clone();
    let mut sz = 1;
    while sz < n {
        let mut lo = 0;
        while lo < n-sz {
            super::merge(v, &mut aux, lo, lo+sz, cmp::min(lo+sz+sz, n));
            lo += sz+sz;
        }
        sz += sz;
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
