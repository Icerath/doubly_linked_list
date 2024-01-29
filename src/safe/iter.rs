use super::DoublyLinkedList;

// IntoIter
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
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}
// Iter
pub struct Iter<'a, T> {
    list: &'a DoublyLinkedList<T>,
    head: usize,
    tail: usize,
    len: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.len = self.len.checked_sub(1)?;
        let val = &self.list.buf[self.head].val;
        self.head = self.list.buf[self.head].next;
        Some(val)
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.len = self.len.checked_sub(1)?;
        let val = &self.list.buf[self.tail].val;
        self.tail = self.list.buf[self.tail].prev;
        Some(val)
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> IntoIterator for &'a DoublyLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            list: self,
            head: self.head,
            tail: self.tail,
            len: self.len(),
        }
    }
}

// IterMut
pub struct IterMut<'a, T> {
    list: &'a mut DoublyLinkedList<T>,
    head: usize,
    tail: usize,
    len: usize,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.len = self.len.checked_sub(1)?;
        let head = self.head;
        self.head = self.list.buf[self.head].next;
        let val = &mut self.list.buf[head].val;

        // This makes me sad
        Some(unsafe { std::mem::transmute(val) })
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.len = self.len.checked_sub(1)?;
        let tail = self.tail;
        self.tail = self.list.buf[self.tail].prev;
        let val = &mut self.list.buf[tail].val;

        // This makes me sad
        Some(unsafe { std::mem::transmute(val) })
    }
}

impl<'a, T> IntoIterator for &'a mut DoublyLinkedList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;
    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len(),
            list: self,
        }
    }
}

// FromIter
impl<T> FromIterator<T> for DoublyLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut list = Self::default();
        list.buf.reserve(iter.size_hint().0);
        for i in iter {
            list.push_back(i);
        }
        list
    }
}
