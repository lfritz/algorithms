//! Algorithm 2.5 Quickort.

/// Sort `v` using a straightforward implementation of quicksort.
pub fn sort<T: Ord + Copy>(v: &mut Vec<T>) {
    let n = v.len();
    sort_part(v, 0, n);
}

fn sort_part<T: Ord + Copy>(v: &mut Vec<T>, lo: usize, hi: usize) {
    if hi <= lo+1 { return; }
    let j = partition(v, lo, hi);
    sort_part(v, lo, j);
    sort_part(v, j+1, hi);
}

fn partition<T: Ord + Copy>(v: &mut Vec<T>, lo: usize, hi: usize) -> usize {
    let mut i = lo;
    let mut j = hi;
    let x = v[lo];
    loop {
        loop { i += 1; if v[i] >= x || i == hi-1 { break; } }
        loop { j -= 1; if x >= v[j] || j == lo   { break; } }
        if i >= j { break; }
        v.swap(i, j);
    }
    v.swap(lo, j);
    return j;
}

#[cfg(test)]
mod tests {
    use super::{partition, sort};
    use super::super::tests::test_sort;

    fn is_partitioned<T: Ord>(v: &Vec<T>,
                              lo: usize,
                              hi: usize,
                              i: usize) -> bool {
        for j in lo..i   { if v[j] > v[i] { return false; } }
        for j in i+1..hi { if v[j] < v[i] { return false; } }
        return true;
    }

    #[test]
    fn is_partitioned_works() {
        //                              0 1 2 3 4
        assert!(   is_partitioned(&vec![2,5,9,7,3], 0, 5, 0));
        assert!(   is_partitioned(&vec![5,2,6,9,7], 0, 5, 2));
        assert!(   is_partitioned(&vec![6,3,4,1,9], 0, 5, 4));
        assert!( ! is_partitioned(&vec![2,5,1,7,3], 0, 5, 0));
        assert!( ! is_partitioned(&vec![5,7,6,9,7], 0, 5, 2));
        assert!( ! is_partitioned(&vec![6,3,4,1,5], 0, 5, 4));
    }

    #[test]
    fn partition_works() {
        let test_data = vec![
            vec![1,9,4,1,9],
            vec![1,9,4,3,1,9],
            vec![1,9,1,5,7,2,3,1,9],
            vec![1,9,9,4,2,6,3,1,9],
            vec![1,9,5,3,4,3,8,1,9],
            vec![1,9,5,3,8,5,6,1,9],
        ];

        for input in test_data {
            let mut output = input.clone();
            let n = output.len();
            let lo = 2;
            let hi = n - 2;

            let i = partition(&mut output, lo, hi);

            for j in 0..lo { assert!(input[j] == output[j]); }
            for j in hi..n { assert!(input[j] == output[j]); }
            assert!(is_partitioned(&output, lo, hi, i));
        }
    }

    #[test]
    fn sort_works() {
        test_sort(sort);
    }
}
