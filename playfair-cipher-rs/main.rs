use std::io;
use std::io::prelude::*;

mod playfair;

fn main() {
    let mut choice = String::new();

    loop {
        println!("Choice: \nencrypt, decrypt, exit");
        println!("Enter  choice: ");
        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        println!("Choice");

        match choice.trim() {
            "encrypt" => encrypt_playfair(),
            "decrypt" => decrypt_playfair(),
            "exit" => break,
            _ => println!("No option"),
        }

        choice.clear();
        println!("---------------------");
    }
}

fn encrypt_playfair() {
    let mut file_name = String::new();
    println!("Enter filename:");
    io::stdin().read_line(&mut file_name)
        .expect("Could not read file name");
    let file_name = file_name.trim();

    use std::fs::File;
    let mut f = File::open(file_name).expect("Unable to open file");
    let mut plain_text = String::new();
    f.read_to_string(&mut plain_text).unwrap();
    let plain_text = plain_text.trim().to_string();

    let mut key = String::new();
    println!("Enter Key :");
    io::stdin().read_line(&mut key);
    let key = key.trim().to_string();

    println!("Plain Text: \n{}", plain_text);
    println!("Key {}", key);

    println!("cipher : {} ", playfair::encrypt(&plain_text, &key));
}

fn decrypt_playfair() {
    let mut file_name = String::new();
    println!("Enter filename:");
    io::stdin().read_line(&mut file_name)
        .expect("Could not read file name");
    let file_name = file_name.trim();

    use std::fs::File;
    let mut f = File::open(file_name).expect("Unable to open file");
    let mut cipher_text = String::new();
    f.read_to_string(&mut cipher_text).unwrap();
    let cipher_text = cipher_text.trim().to_string();

    let mut key = String::new();
    println!("Enter Key :");
    io::stdin().read_line(&mut key);
    let key = key.trim().to_string();

    println!("Cipher Text: \n{}", cipher_text);
    println!("Key {}", key);

    println!("plain : {} ", playfair::decrypt(&cipher_text, &key));
}

