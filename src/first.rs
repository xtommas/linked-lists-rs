// List 1: a bad stack

use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // 'while let' = do this until the pattern doesn't match
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope here and gets dropped;
            // but its node's next field has been set to Link::Empty
            // so no unbounded recursion occurs
        }
    }
}

impl List {
    // static method (doesn't take self as an argument)
    // New
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    // Push operation
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    // Pop operation
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list case
        assert_eq!(list.pop(), None);

        // Add values to the list
        list.push(1);
        list.push(420);
        list.push(69);

        // Check removal
        assert_eq!(list.pop(), Some(69));
        assert_eq!(list.pop(), Some(420));

        // Add more values to make sure is not corrupted
        list.push(77);
        list.push(5);

        // Check removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(77));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
