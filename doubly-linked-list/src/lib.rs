// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

use std::ptr::{self, NonNull};

struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Option<NonNull<Node<T>>>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T> {
    head: Link<T>,
    back: Option<NonNull<Node<T>>>,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    current: Option<NonNull<Node<T>>>,
}

pub struct Iter<'a, T>(&'a Link<T>);

impl<T> Node<T> {
    pub fn boxed(value: T) -> Box<Node<T>> {
        Box::new(Self {
            value,
            next: None,
            prev: None,
        })
    }
}

impl<'a, T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            back: None,
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
            unsafe { Some(NonNull::new_unchecked(head.as_mut())) }
        } else {
            None
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
        unsafe {
            self.current
                .as_mut()
                .map(|current| &mut current.as_mut().value)
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            if let Some(current) = self.current.as_mut() {
                self.current = if let Some(node) = &mut current.as_mut().next {
                    Some(NonNull::new_unchecked(node.as_mut()))
                } else {
                    None
                };
            };
            self.current.as_mut().map(|node| &mut node.as_mut().value)
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            if let Some(current) = self.current.as_mut() {
                self.current = current.as_mut().prev;
                self.current.as_mut().map(|node| &mut node.as_mut().value)
            } else {
                None
            }
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let mut new_current = None;
        let res = unsafe {
            match self.current.as_mut() {
                Some(current) => {
                    let current_mut = current.as_mut();
                    match current_mut.prev.as_mut() {
                        Some(prev) => match current_mut.next.take() {
                            Some(mut next) => {
                                new_current = Some(NonNull::new_unchecked(next.as_mut()));
                                next.prev = Some(*prev);
                                // need to get current from the owner
                                let current = prev.as_mut().next.replace(next);
                                current.map(|node| node.value)
                            }
                            None => {
                                new_current = Some(*prev);
                                self.list.back = Some(*prev);
                                // need to get current from the owner
                                let current = prev.as_mut().next.take();
                                current.map(|node| node.value)
                            }
                        },
                        None => match current.as_mut().next.take() {
                            Some(mut next) => {
                                new_current = Some(NonNull::new_unchecked(next.as_mut()));
                                next.prev = None;
                                // need to get current from the owner
                                let current = self.list.head.replace(next);
                                current.map(|node| node.value)
                            }
                            None => {
                                new_current = None;
                                self.list.back = None;
                                // need to get current from the owner
                                let current = self.list.head.take();
                                current.map(|node| node.value)
                            }
                        },
                    }
                }
                None => None,
            }
        };

        self.current = new_current;

        res
    }

    pub fn insert_after(&mut self, element: T) {
        let mut new_node = Node::boxed(element);
        unsafe {
            match self.current.as_mut() {
                Some(current) => {
                    new_node.prev = Some(*current);
                    if let Some(next) = &mut current.as_mut().next {
                        next.prev = Some(NonNull::new_unchecked(new_node.as_mut()));
                    } else {
                        // end of list
                        self.list.back = Some(NonNull::new_unchecked(new_node.as_mut()));
                    }
                    // new_node now owns the node.next
                    new_node.next = current.as_mut().next.take();
                    current.as_mut().next = Some(new_node);
                }
                None => {
                    // empty list case
                    self.current = Some(NonNull::new_unchecked(new_node.as_mut()));
                    self.list.back = Some(NonNull::new_unchecked(new_node.as_mut()));
                    self.list.head = Some(new_node);
                }
            };
        };
    }

    pub fn insert_before(&mut self, element: T) {
        let mut new_node = Node::boxed(element);
        unsafe {
            match self.current.as_mut() {
                Some(current) => {
                    if let Some(prev) = current.as_mut().prev.as_mut() {
                        new_node.prev = Some(*prev);
                        // need to get current from the owner
                        let current = prev.as_mut().next.take();
                        new_node.next = current;
                        let new_prev = NonNull::new_unchecked(new_node.as_mut());
                        prev.as_mut().next = Some(new_node);
                        *prev = new_prev;
                    } else {
                        // end of list
                        current.as_mut().prev = Some(NonNull::new_unchecked(new_node.as_mut()));
                        new_node.prev = None;
                        // need to get current from the owner
                        let current = self.list.head.take();
                        new_node.next = current;
                        self.list.head = Some(new_node);
                    }
                }
                None => {
                    // empty list case
                    self.current = Some(NonNull::new_unchecked(new_node.as_mut()));
                    self.list.back = Some(NonNull::new_unchecked(new_node.as_mut()));
                    self.list.head = Some(new_node);
                }
            };
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

unsafe impl<T> Send for LinkedList<T> {}
unsafe impl<T> Sync for LinkedList<T> {}
