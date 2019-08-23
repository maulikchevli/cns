use std::io;
use std::io::prelude::*;

mod columnar_transposition;

fn main() {
    let mut choice = String::new();

    loop {
        println!("Columnar Transposition cipher: \nencrypt, decrypt, exit");
        println!("Enter choice: ");
        io::stdin().read_line(&mut choice)
            .expect("Failed to read choice");
        println!("Choice");

        match choice.trim() {
            "encrypt" => encrypt_columnar_transposition(),
            "decrypt" => decrypt_columnar_transposition(),
            "exit" => break,
            _ => println!("Invalid choice!"),
        }

        choice.clear();
        println!("\n===========================\n");
    }
}

fn encrypt_columnar_transposition() {
    let mut file_name = String::new();
    println!("Enter Filename for plaintext.");
    io::stdin().read_line(&mut file_name)
        .expect("Could not read file name");
    let file_name = file_name.trim();

    use std::fs::File;
    let mut f = File::open(file_name)
        .expect("Unable to open file");
    let mut plain_text = String::new();
    f.read_to_string(&mut plain_text).unwrap();

    let mut key = String::new();
    println!("Enter Key: ");
    io::stdin().read_line(&mut key)
        .expect("Could not read Key");
    let key = key.trim().to_string();

    println!("Plain Text: {}", plain_text);
    println!("Key: {}", key);

    println!("Cipher Text: {}", columnar_transposition::encrypt(&plain_text, &key));
}

fn decrypt_columnar_transposition() {
    let mut file_name = String::new();
    println!("Enter file containing encrypted text.");
    io::stdin().read_line(&mut file_name)
        .expect("Could not read file name");
    let file_name = file_name.trim();

    use std::fs::File;
    let mut f = File::open(file_name)
        .expect("Could not open file");
    let mut cipher_text = String::new();
    f.read_to_string(&mut cipher_text).unwrap();

    let mut key = String::new();
    println!("Enter key.");
    io::stdin().read_line(&mut key)
        .expect("Could not read key");
    let key = key.trim().to_string();

    println!("Cipher Text: {}", cipher_text);
    println!("Key: {}", key);

    println!("Plain Text: {}", columnar_transposition::decrypt(&cipher_text, &key));
}
