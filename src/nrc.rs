use std::{collections::HashMap};

fn nrc(word: String) -> () {

    let mut map: HashMap<char, i32> = HashMap::new();
    let mut nrcc = '0';

    for ch in word.chars() {
        map.entry(ch).and_modify(|v| *v += 1).or_insert(1);
    }

    let max_entry = map.iter().max_by_key(|&(_,v)| v);
    if let Some((ch, _)) = max_entry {
        println!("{ch}");
    }

    for (char, f) in map.iter() {
        if *f == 1 {
            nrcc = *char;
        }
    }
    println!("{nrcc}")


}

fn main() {
    nrc(String::from("swiss"));
}