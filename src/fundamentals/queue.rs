//! Algorithm 1.3: FIFO queue.

use std::ops::DerefMut;
use std::ptr;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { value: value, next: next }
    }
}

/// A queue implementation based on a linked list.
pub struct Queue<T> {
    first: Option<Box<Node<T>>>,
    last: * mut Node<T>,
    n: usize,
}

impl<T> Queue<T> {
    /// Constructs a new, empty `Queue<T>`.
    pub fn new() -> Queue<T> {
        Queue { first: None, last: ptr::null_mut(), n: 0 }
    }

    /// Returns `true` if the queue contains no elements.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Returns the number of elements in the queue.
    pub fn len(&self) -> usize {
        self.n
    }

    /// Adds an element to the end of the queue.
    pub fn enqueue(&mut self, value: T) {
        let mut new_element = Box::new(Node::new(value, None));
        if self.is_empty() {
            self.last = new_element.deref_mut();
            self.first = Some(new_element);
        } else {
            let old_last = self.last;
            self.last = new_element.deref_mut();
            unsafe { (*old_last).next = Some(new_element); }
        }
        self.n += 1;
    }

    /// Removes an element from the end of the queue and returns it.
    pub fn dequeue(&mut self) -> Option<T> {
        self.first.take().map(|mut node| {
            self.n -= 1;
            match node.next.take() {
                Some(next_node) => {
                    self.first = Some(next_node);
                },
                None => {
                    self.first = None;
                    self.last = ptr::null_mut();
                },
            }
            node.value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn empty_queue() {
        let q: Queue<isize> = Queue::new();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn single_element() {
        let mut q = Queue::new();
        q.enqueue("hello");
        assert!( ! q.is_empty());
        assert_eq!(q.len(), 1);
        assert_eq!(q.dequeue(), Some("hello"));
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn ten_elements() {
        let mut q = Queue::new();
        for i in 0..10 {
            q.enqueue(i);
        }
        for i in 0..10 {
            assert_eq!(q.dequeue(), Some(i));
        }
        assert_eq!(q.dequeue(), None);
    }
}