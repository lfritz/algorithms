//! Algorithms based on heaps

pub mod pq;
pub mod sort;

fn swim<T: Ord>(v: &mut Vec<T>, i: usize) {
    let mut i = i;
    while i > 0 {
        let parent = (i - 1) / 2;
        if v[i] < v[parent] { break; }
        v.swap(i, parent);
        i = parent;
    }
}

fn sink<T: Ord>(v: &mut Vec<T>, i: usize, n: usize) {
    let mut i = i;
    loop {
        let mut child = 2*i + 1;
        if child >= n { return; }
        if child+1 < n && v[child+1] > v[child] { child += 1; }
        if v[i] > v[child] { return; }
        v.swap(i, child);
        i = child;
    }
}

#[cfg(test)]
mod tests {
    use super::{sink, swim};

    #[test]
    fn swim_works() {
        let mut v = vec![10,6,9,4,3,7,2,1];
        swim(&mut v, 7);
        assert_eq!(vec![10,6,9,4,3,7,2,1], v);

        let mut v = vec![10,6,9,4,3,7,2,5];
        swim(&mut v, 7);
        assert_eq!(vec![10,6,9,5,3,7,2,4], v);

        let mut v = vec![10,6,9,4,3,7,2,8];
        swim(&mut v, 7);
        assert_eq!(vec![10,8,9,6,3,7,2,4], v);

        let mut v = vec![10,6,9,4,3,7,2,20];
        swim(&mut v, 7);
        assert_eq!(vec![20,10,9,6,3,7,2,4], v);
    }

    #[test]
    fn sink_works() {
        let mut v = vec![10,9,4,6,7,3,1];
        sink(&mut v, 0, 7);
        assert_eq!(vec![10,9,4,6,7,3,1], v);

        let mut v = vec![8,9,4,6,7,3,1];
        sink(&mut v, 0, 7);
        assert_eq!(vec![9,8,4,6,7,3,1], v);

        let mut v = vec![5,9,4,6,7,3,1];
        sink(&mut v, 0, 7);
        assert_eq!(vec![9,7,4,6,5,3,1], v);

    }
}
