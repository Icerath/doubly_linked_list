use std::fmt;

use crate::{dll::Node, DoublyLinkedList};

// From Iter
impl<T> FromIterator<T> for DoublyLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ret = Self::new();
        for i in iter {
            ret.push_back(i);
        }
        ret
    }
}

// Into Iter
pub struct IntoIter<T> {
    list: DoublyLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len()
    }
}

impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}
// Iter
type Link<'a, T> = Option<&'a Node<T>>;

#[derive(Debug)]
pub struct Iter<'a, T> {
    head: Link<'a, T>,
    tail: Link<'a, T>,
    len: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.len = self.len.checked_sub(1)?;

        let head = self.head;
        if let Some(head) = head {
            self.head = head.next.map(|ptr| unsafe { ptr.as_ref() });
        }
        head.map(|node| &node.val)
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.len = self.len.checked_sub(1)?;

        let tail = self.tail;
        if let Some(tail) = tail {
            self.tail = tail.prev.map(|ptr| unsafe { ptr.as_ref() });
        }
        tail.map(|node| &node.val)
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head.map(|ptr| unsafe { ptr.as_ref() }),
            tail: self.tail.map(|ptr| unsafe { ptr.as_ref() }),
            len: self.len(),
        }
    }
}

impl<T> Clone for DoublyLinkedList<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

impl<T> fmt::Debug for DoublyLinkedList<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}