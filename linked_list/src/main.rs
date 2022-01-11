// Linked List that stores unsigned (u:32) integers.
// Constant time insertion and deletion (LIFO).
// Keep track of its size and disply function.

// "pointer to a chunk of heap memory(runtime memory)" = Box in Rust

struct Node {
    value: u32,
    next: Option<Box<Node>>, // we use Option as we ain't got null in Rust
}

pub struct LinkedList{
    head: Option<Box<Node>>,
    size: usize
}

impl Node {
    fn new(val: u32, next: Option<Box<Node>>) -> Node {
        Node{value: val, next: next}
    }
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {head: None, size: 0}
    }
    #[allow(unused)]
    fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    #[allow(unused)]
    fn get_size(&self) -> usize {
        self.size
    }
    fn push(&mut self, val:  u32) {
        let new_node = Box::new(Node::new(val, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    #[allow(unused)]
    fn pop(&mut self) -> Option<u32> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
    fn display(&self) {
        let mut current = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            };
        }
        println!("{}", result);
    }
}

fn main(){
    let mut linked_list = LinkedList::new();
    linked_list.push(12);
    linked_list.push(45);
    linked_list.push(42);
    linked_list.push(46);
    linked_list.push(49);
    linked_list.display();
}
