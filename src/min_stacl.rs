struct MinStack {
  stack: Vec<i32>,
  min_stack: Vec<i32>
}

impl MinStack {
  fn new() -> Self {
      Self { stack: Vec::new(), min_stack: Vec::new() }
  }

  fn push(&mut self, val: i32) {
      self.stack.push(val);
      if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
          self.min_stack.push(val);
      }
  }

  fn pop(&mut self) {
      if let Some(top) = self.stack.pop(){
          if top == *self.min_stack.last().unwrap() {
              self.min_stack.pop();
          }
      }
      
  }

  fn top(&self) -> Option<&i32> {
      self.stack.last()
  }

  fn get_min(&self) -> Option<&i32> {
      self.min_stack.last()
  }
}

fn main() {
  let mut s = MinStack::new();
  s.push(5);
  s.push(2);
  s.push(8);
  s.push(1);

  println!("Min: {:?}", s.get_min()); // 1

  s.pop();

  println!("Min after pop: {:?}", s.get_min()); // 2

  println!("Top: {:?}", s.top()); // 8
}
