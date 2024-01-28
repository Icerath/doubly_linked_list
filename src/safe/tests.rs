use super::*;
#[test]
fn test_basics() {
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(1));

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(3));
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn test_iter() {
    let mut items = [1, 2, 3, 4, 5];
    let mut list: DoublyLinkedList<_> = items.into_iter().collect();

    assert!(list.iter().eq(&items));
    assert!(list.iter().rev().eq(items.iter().rev()));

    assert!(list.iter_mut().eq(&mut items));
    assert!(list.iter_mut().rev().eq(items.iter_mut().rev()));

    assert!(list.clone().into_iter().eq(items));
    assert!(list.clone().into_iter().rev().eq(items.into_iter().rev()));

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next_back(), Some(&5));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next_back(), Some(&4));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next_back(), Some(&mut 5));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next_back(), Some(&mut 4));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next_back(), Some(5));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next_back(), Some(4));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
}
