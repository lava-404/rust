struct Node {
    data: i32,
    next: i32
}

struct LinkedList {
    linked_list: Vec<Node>
}


impl LinkedList {
    fn new() -> Self {
       Self { linked_list: vec![] }
    }

    fn add(&mut self, val: i32, addr: i32){
       if let Some(node) = self.linked_list.last_mut(){
        node.next = addr;
        let n  = Node{
            data: val,
            next: 0
        };
        self.linked_list.push(n);
       }
    }
}