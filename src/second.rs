// List 2: an ok singly-linked stack

pub struct List {
    head: Link,
}

// type alias to not have to constantly write 'Option<Box<Node>>'
type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    // static method (doesn't take self as an argument)
    // New
    pub fn new() -> Self {
        List { head: None }
    }
    // Push operation
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    // Pop operation
    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // 'while let' = do this until the pattern doesn't match
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}
