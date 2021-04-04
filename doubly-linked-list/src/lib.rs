// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

use std::ptr;

struct Node<T> {
    value: T,
    next: Link<T>,
    prev: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T> {
    head: Link<T>,
    back: *mut Node<T>,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    current: *mut Node<T>,
}

pub struct Iter<'a, T>(&'a Link<T>);

impl<T> Node<T> {
    pub fn boxed(value: T) -> Box<Node<T>> {
        Box::new(Self {
            value,
            next: None,
            prev: ptr::null_mut(),
        })
    }
}

impl<'a, T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            back: ptr::null_mut(),
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&'a mut self) -> Cursor<'a, T> {
        let current = if let Some(node) = &mut self.head {
            node.as_mut()
        } else {
            ptr::null_mut()
        };

        Cursor {
            list: self,
            current,
        }
    }
    /// Return a cursor positioned on the back element
    pub fn cursor_back(&'a mut self) -> Cursor<'a, T> {
        let current = self.back;

        Cursor {
            list: self,
            current,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter(&self.head)
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            if let Some(node) = self.current.as_mut() {
                self.current = if let Some(node) = &mut node.next {
                    node.as_mut()
                } else {
                    ptr::null_mut()
                };
            };
            self.current.as_mut().map(|node| &mut node.value)
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        unsafe {
            match self.current.as_mut() {
                Some(node) => match node.prev.as_mut() {
                    Some(_) => unimplemented!(),
                    None => match node.next {
                        Some(_) => unimplemented!(),
                        None => self.list.head.take().map(|node| node.value),
                    },
                },
                None => None,
            }
        }
    }

    pub fn insert_after(&mut self, element: T) {
        let mut new_node = Node::boxed(element);
        match unsafe { self.current.as_mut() } {
            Some(node) => {
                new_node.prev = node;
                node.next = Some(new_node);
            }
            None => {
                // empty list case
                self.current = new_node.as_mut();
                self.list.back = new_node.as_mut();
                self.list.head = Some(new_node);
            }
        };
    }

    pub fn insert_before(&mut self, _element: T) {
        unimplemented!()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(node) = self.0 {
            self.0 = &node.next;
            Some(&node.value)
        } else {
            None
        }
    }
}
