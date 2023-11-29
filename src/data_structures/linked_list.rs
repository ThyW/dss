use std::mem;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Box<Self> {
        Box::new(Self { value, next: None })
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    /// Push a `value` to the end of the list.
    pub fn push_back(&mut self, value: T) {
        self.size += 1;
        if self.head.is_none() {
            self.head = Some(Node::new(value));
        } else {
            let mut node = self.head.as_mut().unwrap();

            while node.next.is_some() {
                node = node.next.as_mut().unwrap();
            }

            node.next = Some(Node::new(value));
        }
    }

    /// Push a `value` to the start of the list, making the value a new head.
    pub fn push_front(&mut self, value: T) {
        self.size += 1;
        if self.head.is_none() {
            self.head = Some(Node::new(value));
        } else {
            let mut new_head = Node::new(value);
            mem::swap(&mut new_head.as_mut().next, &mut self.head);
            self.head = Some(new_head);
        }
    }

    /// Get an **immutable** reference to an element located `index` elements from the head .
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            return None;
        }

        let mut i = 0;
        let mut node = self.head.as_ref().unwrap();

        while i != index {
            node = node.next.as_ref().unwrap();
            i += 1;
        }

        Some(&node.value)
    }

    /// Get a **mutable** reference to an element located `index` elements from the head .
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.size {
            return None;
        }

        let mut i = 0;
        let mut node = self.head.as_mut().unwrap();

        while i != index {
            node = node.next.as_mut().unwrap();
            i += 1;
        }

        Some(&mut node.value)
    }

    /// Get the length of the list.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Is the list empty?
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Remove the `head` of the list, returning the value.
    pub fn pop_head(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let mut head = mem::take(&mut self.head);
        mem::swap(&mut self.head, &mut head.as_mut().unwrap().next);
        self.size -= 1;

        Some(head.unwrap().value)
    }

    /// Remove the element furthest from the head, returning it.
    pub fn pop_back(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let mut i = 0;
        let mut node = self.head.as_mut().unwrap();

        while i + 2 < self.size {
            node = node.next.as_mut().unwrap();
            i += 1;
        }

        let n = mem::take(&mut node.next);
        self.size -= 1;

        Some(n.unwrap().value)
    }
}
