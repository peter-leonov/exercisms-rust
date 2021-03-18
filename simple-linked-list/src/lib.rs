use std::iter::FromIterator;

#[derive(PartialEq, Eq)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(PartialEq, Eq)]
pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: Default::default(),
        }
    }

    fn new_with_head(head: Link<T>) -> Self {
        Self { head }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        // I'd expect iter over values to be slower
        // than the loop below, but it gets compiled to this:
        // .LBB3_3:
        // add     rbx, 1
        // mov     r14, qword ptr [r14]
        // test    r14, r14
        // jne     .LBB3_3
        // => ❤️
        self.into_iter().count()

        // and the following hand crafted loop gets compiled to:
        // .LBB3_3:
        // add     r14, 1
        // mov     rbx, qword ptr [rbx]
        // cmp     qword ptr [rbx], 0
        // jne     .LBB3_3

        // let mut len = 0;

        // let mut link = &self.head;
        // while let Some(node) = link {
        //     len += 1;
        //     link = &node.next;
        // }

        // len
    }

    pub fn push(&mut self, value: T) {
        self.head = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
    }

    fn push_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head.take();
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;
            head.value
        })
    }

    fn pop_node(&mut self) -> Link<T> {
        self.head.take().map(|mut head| {
            self.head = head.next.take();
            head
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut new_list = Self::new();
        while let Some(node) = self.pop_node() {
            new_list.push_node(node);
        }
        new_list
    }
}

pub struct SimpleLinkedListIter<T>(SimpleLinkedList<T>);

impl<T> Iterator for SimpleLinkedListIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;
    type IntoIter = SimpleLinkedListIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleLinkedListIter(self.rev())
    }
}

pub struct SimpleLinkedListRefIter<'a, T>(&'a Link<T>);

impl<'a, T> Iterator for SimpleLinkedListRefIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // it's as fast as the if let below 🤯
        self.0.as_ref().map(|node| {
            self.0 = &node.next;
            &node.value
        })
        // if let Some(node) = self.0 {
        //     self.0 = &node.next;
        //     return Some(&node.value);
        // } else {
        //     None
        // }
    }
}

impl<'a, T> IntoIterator for &'a SimpleLinkedList<T> {
    type Item = &'a T;
    type IntoIter = SimpleLinkedListRefIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleLinkedListRefIter(&self.head)
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        Self::new_with_head(
            _iter
                .into_iter()
                .fold(None, |next, value| Some(Box::new(Node { value, next }))),
        )
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
        self.into_iter().collect()
    }
}
