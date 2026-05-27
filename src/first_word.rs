use std::io;

fn first_word() -> String {
    let mut first_word = String::from("");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    for char in input.chars() {
        if char != ' ' {
            first_word += &char.to_string();
        }
        else {
            break;
        }
        
    }
    first_word
}

fn main() {
    let first_word = first_word();
    println!("This is the first word: {first_word}" )
}