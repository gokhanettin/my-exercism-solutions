use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut curr = self.head.as_ref();
        let mut counter = 0usize;
        while let Some(node) = curr {
            counter += 1;
            curr = node.next.as_ref();
        }
        counter
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, element: T) {
        let mut node = Box::new(Node {
            data: element,
            next: None,
        });
        node.next = self.head.take();
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut old = self;
        let mut rev = Self::new();
        while let Some(elem) = old.pop() {
            rev.push(elem);
        }
        rev
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for elem in iter {
            list.push(elem);
        }
        list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(list: SimpleLinkedList<T>) -> Vec<T> {
        let mut old = list;
        let mut vec = Vec::new();
        while let Some(elem) = old.pop() {
            vec.push(elem);
        }
        vec.reverse();
        vec
    }
}
