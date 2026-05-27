use std::io;
use rand::RngExt;
fn main() -> () {
    let mut rep = false;
    let mut first = true;
    let mut rng = rand::rng();

    while rep == true || first == true {
        first = false;
        let random: i32 = rng.random_range(1..30);
    println!("Hello rand no: {}", random);

    println!("guess a number");
    let mut g = String::new();
    io::stdin().read_line(&mut g).unwrap();
    let guess: i32 = g.trim().parse().expect("error parsing");

    if guess > random {
        println!("Too big, try again");
        rep = true;
    }
    else if guess < random {
        println!("Too small, try again");
        rep = true;
    }
    else {
        println!("perfect guess");
        println!("Do you want to Continue?, press any key");
        let mut desc = String::new();

        io::stdin().read_line(&mut desc).unwrap();

        if desc.len() > 0 {
            rep = true;
        }
        else {
            break
        }
    }

    }
    
}