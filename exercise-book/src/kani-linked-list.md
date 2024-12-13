<!-- markdownlint-disable MD033 -->
# Verifying Data Structures with Kani

In this exercise we implement a `remove_at` method for a linked list data structure and will write a Kani harness to prove the correctness of the method.

Custom data structures (trees, graphs, etc.) in Rust often require the use of `unsafe` code for efficient implementation.
Working with raw pointers means that Rust's borrow checker cannot guarantee the correctness of memory access, and Rust cannot reliably clean memory for us.
Getting a code like this right is difficult, and without Rust compiler helping us along the way we would have to rely on testing instead.

Today we will look at a linked list - the simplest data structure that relies pointer manipulation.
The example below is very limited.
The unofficial Rust tutorial - [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html) - explores the production-ready design of a linked list from Rust Standard Library, while [Implementing Vec](https://doc.rust-lang.org/nomicon/vec/vec.html) section of The Rustonomicon explores aspects of `unsafe` Rust use for designing data structures in more detail.
The same principles apply to other, more complex data structures like trees, graphs, queues, etc.

## After completing this exercise you will be able to

- set up Kani support for a project
- write Kani harnesses
- produce Kani playback tests

## Tasks

1. Create a new library project `kani-linked-list`, copy the code from below
2. Set up Kani support for the project
3. Add Kani proof for `remove_at` method.
4. If Kani discovers bugs in the method, then generate playback tests.
5. Fix bugs in the code.

### Starting code

This is a doubly-linked list with a relatively limited API.
You can push and pop elements from both ends of the list (`push_front`, `pop_front`, and `push_back`, `pop_back` respectively), you can access the ends of the list (`front[_mut]` and `back[_mut]`), read list's length, and iterate over it.
The test at the bottom of a snippet demonstrates most of the methods available.

```rust
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
            let mut prev = to_remove.prev?;
            let mut next = to_remove.next?;
            prev.as_mut().next = Some(next);
            next.as_mut().prev = Some(prev);

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

mod proofs {
    use super::*;

    // TODO: write a proof for `DoublyLinkedList::remove_at`
}
```

## Help

### Setting up Kani

<p>
<details>
<summary>
Setting up Cargo
</summary>

```toml
# put this into Cargo.toml
[dev-dependencies]
kani-verifier = "0.56.0"

[dependencies]
# enables autocomplete and code inspections for `kani::*` api
kani = { version = "0.56", git = "https://github.com/model-checking/kani", tag = "kani-0.56.0", optional = true }

# removes warnings about unknown `cfg` attributes
[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(rust_analyzer)', 'cfg(kani)'] }
```

</details>
</p>

<p>
<details>
<summary>
Rust Analyzer project settings
</summary>

For VSCode this should be in `.vscode/settings.json`

```json
{
    "rust-analyzer.cargo.features": ["kani"]
}
```

</details>
</p>

<p>
<details>
<summary>
<em>Optional:</em> You may decide to use nightly Rust if Rust Analyzer doesn't work as expected.
</summary>

Create `rust-toolchain.toml` in project's root

```toml
[toolchain]
channel = "nightly"
```

</details>
</p>

### Writing Kani proofs

<p>
<details>
<summary>
Code snippet to keep Rust Analyzer from showing macro errors for Kani
</summary>

```rust ignore
#[cfg_attr(not(rust_analyzer), cfg(kani))]
mod proofs {
    use super::*;

    #[cfg_attr(not(rust_analyzer), kani::proof)]
    fn kani_harness() {
        todo!();
    }

    #[test]
    fn kani_concrete_playback_xxx() {
        // playback code here
    }
}
```

</details>
</p>

### Other Kani help

<p>
<details>
<summary>
Generating a linked list of random elements.
</summary>

You can make a list by making an array first using `kani::any()`.
Then you can pass the array to a [`from_iter`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html#tymethod.from_iter) method.

```rust ignore
const TOTAL: usize = 10;
let items: [u32; TOTAL] = kani::any();

let mut list = DoublyLinkedList::from_iter(items.iter().copied());
assert_eq!(list.len(), TOTAL);
```

</details>
</p>

<p>
<details>
<summary>
You can use <code>kani::any_where()</code> to generate a value within a specific range.
</summary>

```rust ignore
let x: i32 = kani::any_where(|n| (1..=10).contains(n));
```

</details>
</p>

<p>
<details>
<summary>
If the proof takes a lot of time to run you can introduce the upper unwind limit for loops.
</summary>

```rust ignore
#[cfg_attr(not(rust_analyzer), kani::proof)]
#[cfg_attr(not(rust_analyzer), kani::unwind(20))]
fn long_running_proof() {
}
```

The exact limit is not important.
But by making the list under test shorter you can in turn safely lower the limit while letting the solver observe *all* possible states of the program and complete the verification.

</details>
</p>
