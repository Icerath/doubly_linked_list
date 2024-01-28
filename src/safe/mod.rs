use core::fmt;

use self::iter::{Iter, IterMut};

pub mod iter;
#[cfg(test)]
mod tests;

const NIL: usize = usize::MAX;

#[derive(Clone)]
pub struct DoublyLinkedList<T> {
    pub(crate) buf: Vec<Node<T>>,
    pub(crate) head: usize,
    pub(crate) tail: usize,
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
    pub fn pop_front(&mut self) -> Option<T> {
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
    #[must_use]
    pub fn iter(&self) -> Iter<'_, T> {
        self.into_iter()
    }
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.into_iter()
    }
    fn remove_node(&mut self, ptr: usize) -> Node<T> {
        let end = &self.buf[self.len() - 1];
        let (end_prev, end_next) = (end.prev, end.next);

        if let Some(prev) = self.buf.get_mut(end_prev) {
            prev.next = ptr;
        }
        if let Some(next) = self.buf.get_mut(end_next) {
            next.prev = ptr;
        }
        let node = self.buf.swap_remove(ptr);

        if self.head == self.len() {
            self.head = ptr;
        }
        if self.tail == self.len() {
            self.tail = ptr;
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

impl<T> fmt::Debug for DoublyLinkedList<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

#[derive(Clone)]
pub(crate) struct Node<T> {
    pub(crate) val: T,
    pub(crate) next: usize,
    pub(crate) prev: usize,
}
