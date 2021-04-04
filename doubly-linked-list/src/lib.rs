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
        let current = if let Some(head) = &mut self.head {
            head.as_mut()
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

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while let Some(_) = cursor.take() {}
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.current.as_mut().map(|current| &mut current.value) }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            if let Some(current) = self.current.as_mut() {
                self.current = if let Some(node) = &mut current.next {
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
        unsafe {
            if let Some(current) = self.current.as_mut() {
                self.current = current.prev;
                self.current.as_mut().map(|node| &mut node.value)
            } else {
                None
            }
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        unsafe {
            match self.current.as_mut() {
                Some(current) => match current.prev.as_mut() {
                    Some(prev) => match current.next.take() {
                        Some(mut next) => {
                            self.current = next.as_mut();
                            next.prev = prev;
                            // need to get current from the owner
                            let current = prev.next.replace(next);
                            current.map(|node| node.value)
                        }
                        None => {
                            self.current = prev;
                            self.list.back = prev;
                            // need to get current from the owner
                            let current = prev.next.take();
                            current.map(|node| node.value)
                        }
                    },
                    None => match current.next.take() {
                        Some(mut next) => {
                            self.current = next.as_mut();
                            next.prev = ptr::null_mut();
                            // need to get current from the owner
                            let current = self.list.head.replace(next);
                            current.map(|node| node.value)
                        }
                        None => {
                            self.current = ptr::null_mut();
                            self.list.back = ptr::null_mut();
                            // need to get current from the owner
                            let current = self.list.head.take();
                            current.map(|node| node.value)
                        }
                    },
                },
                None => None,
            }
        }
    }

    pub fn insert_after(&mut self, element: T) {
        let mut new_node = Node::boxed(element);
        match unsafe { self.current.as_mut() } {
            Some(current) => {
                new_node.prev = current;
                if let Some(next) = &mut current.next {
                    next.prev = new_node.as_mut();
                } else {
                    // end of list
                    self.list.back = new_node.as_mut();
                }
                // new_node now owns the node.next
                new_node.next = current.next.take();
                current.next = Some(new_node);
            }
            None => {
                // empty list case
                self.current = new_node.as_mut();
                self.list.back = new_node.as_mut();
                self.list.head = Some(new_node);
            }
        };
    }

    pub fn insert_before(&mut self, element: T) {
        let mut new_node = Node::boxed(element);
        match unsafe { self.current.as_mut() } {
            Some(current) => {
                if let Some(prev) = unsafe { current.prev.as_mut() } {
                    current.prev = new_node.as_mut();
                    new_node.prev = prev;
                    // need to get current from the owner
                    let current = prev.next.take();
                    new_node.next = current;
                    prev.next = Some(new_node);
                } else {
                    // end of list
                    current.prev = new_node.as_mut();
                    new_node.prev = ptr::null_mut();
                    // need to get current from the owner
                    let current = self.list.head.take();
                    new_node.next = current;
                    self.list.head = Some(new_node);
                }
            }
            None => {
                // empty list case
                self.current = new_node.as_mut();
                self.list.back = new_node.as_mut();
                self.list.head = Some(new_node);
            }
        };
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
