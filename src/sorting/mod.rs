//! Implementations of data structures and algorithms from Chapter 2: Sorting.

pub mod heap;
pub mod insertion;
pub mod merge;
pub mod quick;
pub mod selection;
pub mod shell;

/// True if `v` is sorted in ascending order.
pub fn is_sorted<T: Ord>(v: &Vec<T>) -> bool {
    for win in v.windows(2) {
        if win[1] < win[0] {
            return false
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::is_sorted;

    pub fn test_sort<F>(sort: F)
        where F: Fn(&mut Vec<isize>) -> () {
        let test_data = vec![
            (vec![],              vec![]),
            (vec![42],            vec![42]),
            (vec![2, 3],          vec![2, 3]),
            (vec![2, 3],          vec![3, 2]),
            (vec![1, 2, 3, 4, 5], vec![4, 1, 2, 5, 3]),
        ];
        for pair in test_data {
            let mut input= pair.1;
            sort(&mut input);
            assert_eq!(input, pair.0);
        }
    }
    #[test]
    fn is_sorted_works() {
        assert!(is_sorted(&vec![-9, -8, -6, -2, 0, 2, 2, 5, 6, 9]));
        assert!( ! is_sorted(&vec![-4, -3, -1, 2, 1, 4, 5, 7, 8, 9]));
    }
}
