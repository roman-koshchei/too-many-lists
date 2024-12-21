use std::mem::{self};

//

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    value: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        return List { head: Link::Empty };
    }

    pub fn push(&mut self, value: i32) {
        self.head = Link::More(Box::new(Node {
            value,
            next: mem::replace(&mut self.head, Link::Empty),
        }))
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

// Otherwise it's recursive and will blow up the stack
impl Drop for List {
    fn drop(&mut self) {
        let mut current = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut box_node) = current {
            current = mem::replace(&mut box_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
