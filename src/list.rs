use std::{fmt, ptr::NonNull};

type Link<T> = Option<NonNull<Node<T>>>;

pub struct DoublyLinkedList<T> {
    pub(crate) head: Link<T>,
    pub(crate) tail: Link<T>,
    pub(crate) len: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_back(&mut self, val: T) {
        self.len += 1;

        let Some(mut tail) = self.tail else {
            let node = Node::new(val).alloc();
            self.tail = Some(node);
            self.head = Some(node);
            return;
        };
        let node = Node {
            val,
            prev: Some(tail),
            next: None,
        }
        .alloc();
        unsafe { tail.as_mut() }.next = Some(node);
        self.tail = Some(node);
    }
    pub fn push_front(&mut self, val: T) {
        self.len += 1;

        let Some(mut head) = self.head else {
            let node = Node::new(val).alloc();
            self.head = Some(node);
            self.tail = Some(node);
            return;
        };
        let node = Node {
            val,
            next: Some(head),
            prev: None,
        }
        .alloc();
        unsafe { head.as_mut() }.prev = Some(node);
        self.head = Some(node);
    }
    pub fn pop_back(&mut self) -> Option<T> {
        let mut tail = self.tail?;
        self.len -= 1;

        let new_tail = unsafe { tail.as_mut() }.prev;
        if let Some(mut new_tail) = new_tail {
            unsafe { new_tail.as_mut() }.next = None;
        }
        self.tail = new_tail;
        if new_tail.is_none() {
            self.head = None;
        }
        let node = unsafe { tail.as_ptr().read() };
        unsafe { dealloc(tail) };
        Some(node.val)
    }
    pub fn pop_front(&mut self) -> Option<T> {
        let mut head = self.head?;
        self.len -= 1;
        let new_head = unsafe { head.as_mut() }.next;
        if let Some(mut new_head) = new_head {
            unsafe { new_head.as_mut() }.prev = None;
        }
        self.head = new_head;
        if new_head.is_none() {
            self.tail = None;
        }
        let node = unsafe { head.as_ptr().read() };
        unsafe { dealloc(head) };
        Some(node.val)
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
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

#[derive(Debug)]
pub(crate) struct Node<T> {
    pub(crate) val: T,
    pub(crate) next: Link<T>,
    pub(crate) prev: Link<T>,
}

impl<T> Node<T> {
    pub(crate) fn new(val: T) -> Self {
        Self {
            val,
            next: None,
            prev: None,
        }
    }
    pub(crate) fn alloc(self) -> NonNull<Self> {
        NonNull::from(Box::leak(Box::new(self)))
    }
}

pub(crate) unsafe fn dealloc<T>(node: NonNull<Node<T>>) {
    let _ = unsafe { Box::from_raw(node.as_ptr()) };
}
