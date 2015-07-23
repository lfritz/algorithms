//! Algorithm 2.4 Top-down merge sort.

/// Sort `v` using a top-down merge sort.
pub fn sort<T: Ord + Copy>(v: &mut Vec<T>) {
    let n = v.len();
    let mut aux = v.clone();
    sort_part(v, &mut aux, 0, n);
}

// Sort `v[lo..hi]`, using `aux` for temporary storage.
fn sort_part<T: Ord + Copy>(v: &mut Vec<T>, aux: &mut Vec<T>,
                            lo: usize, hi: usize) {
    if hi < lo+2 {
        return;
    }
    let mid = lo + (hi - lo)/2;
    sort_part(v, aux, lo, mid);
    sort_part(v, aux, mid, hi);
    super::merge(v, aux, lo, mid, hi);
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
