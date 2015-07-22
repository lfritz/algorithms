//! Algorithm 1.2: Pushdown stack.

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { value: value, next: next }
    }
}

pub struct Stack<T> {
    first: Option<Box<Node<T>>>,
    n: usize,
}

/// A stack implementation based on a linked list.
impl <T> Stack<T> {
    /// Constructs a new, empty stack.
    pub fn new() -> Stack<T> {
        Stack { first: None, n: 0 }
    }

    /// Returns `true` if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Returns the number of elements in the stack.
    pub fn size(&self) -> usize {
        self.n
    }

    /// Adds an element to the stack.
    pub fn push(&mut self, value: T) {
        self.first = Some(Box::new(Node::new(value, self.first.take())));
        self.n += 1;
    }

    /// Removes an element from the stack and returns it.
    pub fn pop(&mut self) -> Option<T> {
        self.first.take().map(|mut node| {
            self.first = node.next.take();
            self.n -= 1;
            node.value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn empty_stack() {
        let mut s: Stack<isize> = Stack::new();
        assert!(s.is_empty());
        assert_eq!(s.size(), 0);
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn single_element() {
        let mut s = Stack::new();
        s.push("hello");
        assert!( ! s.is_empty());
        assert_eq!(s.size(), 1);
        assert_eq!(s.pop(), Some("hello"));
        assert!(s.is_empty());
        assert_eq!(s.size(), 0);
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn ten_elements() {
        let mut s = Stack::new();
        for i in 0..10 {
            s.push(i);
        }
        for i in 0..10 {
            assert_eq!(s.pop(), Some(9 - i));
        }
        assert_eq!(s.pop(), None);
    }
}
