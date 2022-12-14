use std::io;

fn main () { 
    println!("Input the string you want to encrypt:");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Cannot read input");
    println!("You encrypted string: {}", user_input);
}
