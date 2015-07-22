//! Implementations of merge sort.

pub mod bottom_up;
pub mod top_down;

// Merge v[lo..mid] with v[mid..hi], using `aux` for temporary storage.
fn merge<T: Ord + Copy>(v: &mut Vec<T>, aux: &mut Vec<T>,
                        lo:usize, mid:usize, hi:usize) {
    // copy values to `aux`
    for k in lo..hi {
        aux[k] = v[k];
    }

    // merge back to `v`
    let mut i = lo;
    let mut j = mid;
    for k in lo..hi {
        if      i >= mid        { v[k] = aux[j]; j += 1; }
        else if j >= hi         { v[k] = aux[i]; i += 1; }
        else if aux[j] < aux[i] { v[k] = aux[j]; j += 1; }
        else                    { v[k] = aux[i]; i += 1; }
    }
}

#[cfg(test)]
mod tests {
    use super::merge;

    #[test]
    fn merge_works() {
        //                                                 0 1 2 3 4 5 6 7
        let mut aux = vec![0; 8]; let mut v =         vec![2,5,7,4,1,3,8,9];
        merge(&mut v, &mut aux, 2,3,4); assert_eq!(v, vec![2,5,4,7,1,3,8,9]);
        merge(&mut v, &mut aux, 0,2,4); assert_eq!(v, vec![2,4,5,7,1,3,8,9]);
        merge(&mut v, &mut aux, 0,4,8); assert_eq!(v, vec![1,2,3,4,5,7,8,9]);
    }
}
