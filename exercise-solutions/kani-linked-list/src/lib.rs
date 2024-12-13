use std::{fmt::Debug, ptr::NonNull, vec::IntoIter};

type Link<T> = Option<NonNull<Node<T>>>;

pub struct DoublyLinkedList<T> {
    first: Link<T>,
    last: Link<T>,
    len: usize,
}

struct Node<T> {
    prev: Link<T>,
    next: Link<T>,
    elem: T,
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let mut new_first = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                prev: None,
                next: None,
                elem,
            })));
            match self.first {
                Some(mut old_first) => {
                    // rewire pointers
                    old_first.as_mut().prev = Some(new_first);
                    new_first.as_mut().next = Some(old_first);
                }
                None => {
                    // make a list with a single element
                    self.last = Some(new_first)
                }
            }
            self.first = Some(new_first);
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, elem: T) {
        unsafe {
            let mut new_last = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                prev: None,
                next: None,
                elem,
            })));
            match self.last {
                Some(mut old_last) => {
                    // Put the new back before the old one
                    old_last.as_mut().next = Some(new_last);
                    new_last.as_mut().prev = Some(old_last);
                }
                None => {
                    // make a list with a single element
                    self.first = Some(new_last);
                }
            }
            self.last = Some(new_last);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node = self.first?;
        unsafe {
            let node = Box::from_raw(node.as_ptr());
            let elem = node.elem;

            self.first = node.next;
            match self.first {
                Some(mut new_first) => {
                    new_first.as_mut().prev = None;
                }
                None => {
                    self.last = None;
                }
            }

            self.len -= 1;
            Some(elem)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let node = self.last?;
        unsafe {
            let node = Box::from_raw(node.as_ptr());
            let elem = node.elem;

            self.last = node.prev;
            match self.last {
                Some(mut new_last) => {
                    new_last.as_mut().prev = None;
                }
                None => {
                    self.last = None;
                }
            }

            self.len -= 1;
            Some(elem)
        }
    }

    pub fn front(&self) -> Option<&T> {
        Some(unsafe { &self.first?.as_ref().elem })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        Some(unsafe { &mut self.first?.as_mut().elem })
    }

    pub fn back(&self) -> Option<&T> {
        Some(unsafe { &self.last?.as_ref().elem })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        Some(unsafe { &mut self.last?.as_mut().elem })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> IntoIter<&T> {
        self.into_iter()
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        unsafe {
            // find an element to remove.
            // if `index` is too large this will return `None` early
            let to_remove = {
                let mut to_remove = self.first;
                for _ in 0..index {
                    to_remove = to_remove?.as_mut().next;
                }
                Box::from_raw(to_remove?.as_ptr())
            };
            // connect previous and next elements together
            let prev = to_remove.prev;
            let next = to_remove.next;
            match (prev, next) {
                (Some(mut prev), Some(mut next)) => {
                    prev.as_mut().next = Some(next);
                    next.as_mut().prev = Some(prev);
                }
                (Some(mut prev), None) => {
                    prev.as_mut().next = None;
                    self.last = Some(prev);
                }
                (None, Some(mut next)) => {
                    next.as_mut().prev = None;
                    self.first = Some(next);
                }
                (None, None) => {
                    self.first = None;
                    self.last = None;
                }
            }
            Some(to_remove.elem)
        }
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = T;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        // lazy implementation: we take all items from a linked list, put them
        // into a vector, and return Vec's iterator.
        let mut iter = vec![];
        while let Some(elem) = self.pop_front() {
            iter.push(elem);
        }
        iter.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a DoublyLinkedList<T> {
    type Item = &'a T;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // lazy implementation: we take all items from a linked list, put them
        // into a vector, and return Vec's iterator.
        let mut iter: Vec<&'a T> = vec![];
        let mut current = self.first;
        while let Some(node) = current.map(|n| unsafe { n.as_ref() }) {
            current = node.next;
            iter.push(&node.elem);
        }
        iter.into_iter()
    }
}

impl<T> FromIterator<T> for DoublyLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for item in iter {
            list.push_back(item);
        }
        list
    }
}

// Needed for testing
impl<T> Debug for DoublyLinkedList<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

// Needed for testing
impl<T> PartialEq for DoublyLinkedList<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

#[cfg(test)]
mod tests {
    use crate::DoublyLinkedList;

    #[test]
    fn test_list_apis() {
        let mut list = DoublyLinkedList::new();
        list.push_back(2);
        list.push_front(1);
        list.push_back(3);
        assert_eq!(format!("{list:?}"), "[1, 2, 3]");
        let front = list.front();
        assert_eq!(front, Some(&1));

        let removed = list.remove_at(1);
        assert_eq!(removed, Some(2));
        assert_eq!(format!("{list:?}"), "[1, 3]");

        let non_existing = list.remove_at(100);
        assert_eq!(non_existing, None);
        assert_eq!(format!("{list:?}"), "[1, 3]");
    }
}

#[cfg_attr(not(rust_analyzer), cfg(kani))]
mod proofs {
    use super::*;

    #[cfg_attr(not(rust_analyzer), kani::proof)]
    #[cfg_attr(not(rust_analyzer), kani::unwind(20))]
    fn remove_at() {
        const TOTAL: usize = 10;
        let items: [u32; TOTAL] = kani::any();

        let mut list = DoublyLinkedList::from_iter(items.iter().copied());
        assert_eq!(list.len(), TOTAL);

        // let position: usize = kani::any_where(|&n| n > 0 && n < TOTAL - 1);
        let position: usize = kani::any_where(|n| (0..TOTAL).contains(n));
        let removed = list.remove_at(position);
        assert_eq!(removed, Some(items[position]));
        // Check pointer integrity
        assert!(list.front().is_some());
        assert!(list.back().is_some());

        let position: usize = kani::any_where(|&n| n >= TOTAL);
        let removed = list.remove_at(position);
        assert_eq!(removed, None);
    }

    #[cfg_attr(not(rust_analyzer), kani::proof)]
    fn remove_the_only_element_from_list() {
        let item: u32 = kani::any();
        let mut list = DoublyLinkedList::new();
        list.push_back(item);
        let removed = list.remove_at(0);
        assert_eq!(removed, Some(item));
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);
    }

    // The exact number of playback tests can vary.
    // As you fix more bugs some older playback tests may become invalid, and
    // may have to be removed
    #[test]
    fn kani_concrete_playback_remove_at_11026539725679402838() {
        let concrete_vals: Vec<Vec<u8>> = vec![
            // 4294967292
            vec![252, 255, 255, 255],
            // 4294967292
            vec![252, 255, 255, 255],
            // 4294967292
            vec![252, 255, 255, 255],
            // 4294967292
            vec![252, 255, 255, 255],
            // 4294967292
            vec![252, 255, 255, 255],
            // 4294967292
            vec![252, 255, 255, 255],
            // 4294967292
            vec![252, 255, 255, 255],
            // 4294967292
            vec![252, 255, 255, 255],
            // 4294967292
            vec![252, 255, 255, 255],
            // 4294967292
            vec![252, 255, 255, 255],
            // 0ul
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            // 10ul
            vec![10, 0, 0, 0, 0, 0, 0, 0],
        ];
        kani::concrete_playback_run(concrete_vals, remove_at);
    }
}
