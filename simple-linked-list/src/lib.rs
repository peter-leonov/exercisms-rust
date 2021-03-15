use std::iter::FromIterator;

#[derive(PartialEq, Eq)]
struct Node<T> {
    value: T,
    next: Option<Box<Self>>,
}

#[derive(PartialEq, Eq)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: Default::default(),
        }
    }

    fn new_with_head(head: Option<Box<Node<T>>>) -> Self {
        Self { head }
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

        let mut ptr = &self.head;
        while let Some(node) = ptr {
            len += 1;
            ptr = &node.next;
        }

        len
    }

    pub fn push(&mut self, value: T) {
        let old_head = self.head.take();
        let new_head_node = Box::new(Node {
            value,
            next: old_head,
        });
        self.head = Some(new_head_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take();
        if let Some(old_head_node) = old_head {
            self.head = old_head_node.next;
            Some(old_head_node.value)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut head = None;
        for value in _iter {
            head = Some(Box::new(Node { value, next: head }));
        }
        Self::new_with_head(head)
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
