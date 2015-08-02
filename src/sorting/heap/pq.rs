//! Algorithm 2.6 Heap priority queue

use super::{sink, swim};

pub struct MaxPQ<T: Ord>(Vec<T>);

impl <T: Ord> MaxPQ<T> {
    pub fn new() -> MaxPQ<T> {
        MaxPQ(vec![])
    }

    pub fn insert(&mut self, value: T) {
        let MaxPQ(ref mut v) = *self;
        let n = v.len();
        v.push(value);
        swim(v, n);
    }

    pub fn pop_max(&mut self) -> Option<T> {
        let MaxPQ(ref mut v) = *self;
        let n = v.len() - 1;
        v.swap(0, n);
        let result = v.pop();
        let n = v.len();
        sink(v, 0, n);
        result
    }

    pub fn is_empty(&self) -> bool {
        let MaxPQ(ref v) = *self;
        v.is_empty()
    }

    pub fn size(&self) -> usize {
        let MaxPQ(ref v) = *self;
        v.len()
    }
}

#[cfg(test)]
mod tests {
    use super::MaxPQ;

    #[test]
    fn priority_queue_works_for_1_element() {
        let mut pq = MaxPQ::new();
        assert!(pq.is_empty());
        assert_eq!(0, pq.size());

        pq.insert(42);
        assert!( ! pq.is_empty());
        assert_eq!(1, pq.size());

        assert_eq!(Some(42), pq.pop_max());
        assert!(pq.is_empty());
        assert_eq!(0, pq.size());
    }

    #[test]
    fn priority_queue_works_for_10_elements() {
        let input = vec![5,2,8,7,4,9,0,1,3,6];
        let expected = vec![9,8,7,6,5,4,3,2,1,0];
        let mut output = vec![];
        let mut pq = MaxPQ::new();
        for i in input {
            pq.insert(i);
        }
        assert!( ! pq.is_empty());
        assert_eq!(10, pq.size());
        for _ in 0..10 {
            output.push(pq.pop_max().unwrap());
        }
        assert_eq!(expected, output);
        assert!(pq.is_empty());
        assert_eq!(0, pq.size());
    }
}
