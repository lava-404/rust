fn longest_word(sentence:  String) -> String {
  let mut current = String::from("");
  let mut  longest = String::from("");

  for char in sentence.chars() {
      if char.is_whitespace() || char == ','{
          if current.len() > longest.len() {
              longest = current.clone();
          }
          current.clear();
      }
      else{
          current.push(char);
      }
  }

  if current.len() > longest.len(){
      longest = current;
  }
  longest
}

fn main(){
  longest_word(String::from("Hello World"));
}