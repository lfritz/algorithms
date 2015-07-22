//! Algorithm 2.2: Insertion sort.

/// Sort `v` using insertion sort.
pub fn sort<T: Ord>(v: &mut Vec<T>) {
    let n = v.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && v[j] < v[j-1] {
            v.swap(j, j-1);
            j -= 1;
        }
    }
}

#[test]
fn sort_works() {
    super::tests::test_sort(sort);
}
