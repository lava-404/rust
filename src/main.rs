struct Node {
    value: i32,
    next: Option<Box<Node>>
}

struct LinkedList {
    head: Option<Box<Node>>
}


impl LinkedList{
    fn new() -> Self {
       
        LinkedList { head:  None}
    }

    fn add(&mut self, val: i32){
        if !self.head.is_some(){
            self.head = Some(Box::new(Node {
                value: val,
                next: None

            }));
        return;
        }

        let mut current = self.head.as_mut();
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = Some(Box::new(Node { value: val, next: None }))
                break;
            }
            else{
                current = node.next.as_mut();
            }
        }
    }
    fn get_last_node(&mut self)-> Option<&mut Box<Node>>{
        let mut current = self.head.as_mut();
        let mut last_node = None;
        while let Some(node) = current {
            if node.next.is_none(){
                last_node = Some(node);
                break;
            }
            else{
                current = node.next.as_mut();
            }
        }
        last_node
    }

    fn remove(&mut self){
        if self.head.is_none() {
            return;
        }
    
        // single-node list
        if self.head.as_ref().unwrap().next.is_none() {
            self.head = None;
            return;
        }
    
        let mut current = self.head.as_mut();
    
        while let Some(node) = current {
    
            // if next node is the LAST node
            if node.next.as_ref().unwrap().next.is_none() {
                node.next = None;
                break;
            }
    
            current = node.next.as_mut();
        }
    }

}

