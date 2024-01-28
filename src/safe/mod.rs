#[cfg(test)]
mod tests;
use core::fmt;

const NIL: usize = usize::MAX;

pub struct DoublyLinkedList<T> {
    buf: Vec<Node<T>>,
    head: usize,
    tail: usize,
}

impl<T> DoublyLinkedList<T> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_back(&mut self, val: T) {
        let ptr = self.push_buf(Node {
            val,
            prev: self.tail,
            next: NIL,
        });
        if self.head == NIL {
            self.head = ptr;
        }
        if self.tail != NIL {
            self.buf[self.tail].next = ptr;
        }
        self.tail = ptr;
    }
    pub fn push_front(&mut self, val: T) {
        let ptr = self.push_buf(Node {
            val,
            next: self.head,
            prev: NIL,
        });
        if self.tail == NIL {
            self.tail = ptr;
        }
        if self.head != NIL {
            self.buf[self.head].prev = ptr;
        }
        self.head = ptr;
    }
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let node = self.remove_node(self.tail);

        self.tail = node.prev;
        if self.tail == NIL {
            self.head = NIL;
        } else {
            self.buf[self.tail].next = NIL;
        }
        Some(node.val)
    }
    pub fn pop_front(&mut self) -> Option<T>
    where
        T: fmt::Debug,
    {
        if self.is_empty() {
            return None;
        }
        let node = self.remove_node(self.head);
        self.head = node.next;
        if self.head == NIL {
            self.tail = NIL;
        } else {
            self.buf[self.head].prev = NIL;
        }
        Some(node.val)
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn remove_node(&mut self, ptr: usize) -> Node<T> {
        let end = &self.buf[self.len() - 1];
        let (end_prev, end_next) = (end.prev, end.next);

        if let Some(next) = self.buf.get_mut(end_next) {
            next.prev = ptr;
        }
        if let Some(prev) = self.buf.get_mut(end_prev) {
            prev.next = ptr;
        }

        let node = self.buf.swap_remove(ptr);
        if self.is_empty() {
            self.head = NIL;
            self.tail = NIL;
            return node;
        }
        node
    }
    fn push_buf(&mut self, node: Node<T>) -> usize {
        let ptr = self.buf.len();
        self.buf.push(node);

        ptr
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self {
            buf: vec![],
            head: NIL,
            tail: NIL,
        }
    }
}

#[derive(Debug)]
pub struct Node<T> {
    val: T,
    next: usize,
    prev: usize,
}
