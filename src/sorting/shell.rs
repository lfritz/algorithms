//! Algorithm 2.3 Shell sort.

/// Sort `v` using shell sort.
pub fn sort<T: Ord>(v: &mut Vec<T>) {
    let n = v.len();

    let mut h = 1;
    while h < n/3 {
        h = 3*h + 1;
    }

    while h >= 1 {
        for i in h..n {
            let mut j = i;
            while j >= h && v[j] < v[j-h] {
                v.swap(j, j-h);
                j -= h;
            }
        }
        h /= 3;
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
