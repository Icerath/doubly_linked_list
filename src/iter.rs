use crate::{list::Node, DoublyLinkedList};
use std::{marker::PhantomData, ptr::NonNull};

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
type Link<'a, T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
pub struct Iter<'a, T> {
    head: Link<'a, T>,
    tail: Link<'a, T>,
    len: usize,
    _lifetime: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.len = self.len.checked_sub(1)?;
        let node = self.head;
        self.head = self.head.and_then(|head| unsafe { head.as_ref() }.next);
        node.map(|node| &unsafe { node.as_ref() }.val)
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.len = self.len.checked_sub(1)?;
        let node = self.tail;
        self.tail = self.tail.and_then(|tail| unsafe { tail.as_ref() }.prev);
        node.map(|node| &unsafe { node.as_ref() }.val)
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len(),
            _lifetime: PhantomData,
        }
    }
}

// Iter mut
type LinkMut<T> = Option<NonNull<Node<T>>>;

pub struct IterMut<'a, T> {
    head: LinkMut<T>,
    tail: LinkMut<T>,
    len: usize,
    _lifetime: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.len = self.len.checked_sub(1)?;
        let node = self.head;
        self.head = self.head.and_then(|head| unsafe { head.as_ref() }.next);
        node.map(|mut node| &mut unsafe { node.as_mut() }.val)
    }
}

impl<T> DoubleEndedIterator for IterMut<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.len = self.len.checked_sub(1)?;
        let node = self.tail;
        self.tail = self.tail.and_then(|tail| unsafe { tail.as_ref() }.prev);
        node.map(|mut node| &mut unsafe { node.as_mut() }.val)
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len(),
            _lifetime: PhantomData,
        }
    }
}
