use std::iter::FromIterator;

#[derive(PartialEq, Eq)]
pub struct SimpleLinkedListNode<T> {
    value: T,
    next: Option<Box<Self>>,
}

#[derive(PartialEq, Eq)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<SimpleLinkedListNode<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        match self.head {
            None => true,
            _ => false,
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;

        match &self.head {
            None => (),
            Some(node) => {
                let mut node = node;
                len += 1;
                while let Some(next) = &node.next {
                    len += 1;
                    node = next;
                }
            }
        }

        len
    }

    pub fn push(&mut self, _element: T) {
        let elem = Box::new(SimpleLinkedListNode {
            value: _element,
            next: None,
        });

        match &mut self.head {
            None => {
                self.head = Some(elem);
            }
            Some(next) => {
                if let Some(next) = &mut next.next {
                    if let Some(next) = &mut next.next {
                        if let Some(next) = &mut next.next {
                            if let Some(next) = &mut next.next {
                                if let Some(_) = &mut next.next {
                                    unimplemented!();
                                } else {
                                    next.next = Some(elem);
                                }
                            } else {
                                next.next = Some(elem);
                            }
                        } else {
                            next.next = Some(elem);
                        }
                    } else {
                        next.next = Some(elem);
                    }
                } else {
                    next.next = Some(elem);
                }
                // while let Some(next) = &mut last.next {
                //     last = next;
                // }
                // last.next = Some(elem);
            }
        }

        // if let Some(&mut last) = &self.head {
        //     while let Some(next) = &last.next {
        //         last = next;
        //     }
        //     // last.next = Some(elem);
        // }
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
