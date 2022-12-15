use std::io;

pub mod encryptor;

fn main () { 
    use encryptor::Encryptable;
    println!("Input the string that you want to encrypt:");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Cannot read input");
    println!(   
        "You encrypted string: {}", 
        encryptor::rot13::Rot13(user_input).encrypt()
    );
}
