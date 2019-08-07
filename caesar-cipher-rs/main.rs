use std::io;
use std::io::prelude::*;

mod caesar;

fn main() {
    let mut choice = String::new();

    loop {
        println!("CHoice: \nencrypt, decrypt, brute force, frequency analysis, exit");
        println!("Enter Choice: ");
        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        println!("choice: {}", choice);

        // Input thru stdin contains new line character 
        match choice.trim() {
            "encrypt" => encrypt_caesar(),
            "decrypt" => decrypt_caesar(),
            "brute force" => brute_caesar(),
            "frequency" => frequency_analysis(),
            "exit" => break,
            _ => println!("No option"),
        }

        choice.clear();
        println!("---------------");
   }
}

fn frequency_analysis() {
    let mut file_name = String::new();
    println!("Enter filename:");
    io::stdin().read_line(&mut file_name)
        .expect("Plain Text");

    let file_name = file_name.trim();

    use std::fs::File;
    let mut f = File::open(file_name).expect("Unable to open");
    let mut cipher_text = String::new();
    f.read_to_string(&mut cipher_text).unwrap();

    print!("Cipher text: \n{}", cipher_text);

    print!("Plain text by Frequency analysis: \n{}", caesar::frequency_analysis(&cipher_text));
}

fn brute_caesar() {
    let mut file_name = String::new();
    println!("Enter filename:");
    io::stdin().read_line(&mut file_name)
        .expect("Plain Text");

    let file_name = file_name.trim();

    use std::fs::File;
    let mut f = File::open(file_name).expect("Unable to open");
    let mut cipher_text = String::new();
    f.read_to_string(&mut cipher_text).unwrap();

    print!("Cipher text: \n{}", cipher_text);

    print!("Brute Force::");
    for key in 1..27 {
        println!("-------------------");
        println!("Key {}", key);
        println!("{}", caesar::decipher(&cipher_text, key));
    }
}

fn encrypt_caesar() {
    let mut key = String::new();
    println!("Enter key value:");
    io::stdin().read_line(&mut key)
        .expect("Failed to read key");
    // Convert to u8
    let key: u8 = key.trim().parse().expect("Invalid Input");

    let mut file_name = String::new();
    println!("Enter filename:");
    io::stdin().read_line(&mut file_name)
        .expect("Plain Text");

    let file_name = file_name.trim();

    use std::fs::File;
    let mut f = File::open(file_name).expect("Unable to open");
    let mut plain_text = String::new();
    f.read_to_string(&mut plain_text).unwrap();

    print!("plain text: \n{}", plain_text);

    print!("Cipher Text:\n{}", caesar::cipher(&plain_text, key));
}

fn decrypt_caesar() {
    let mut key = String::new();
    println!("Enter key value:");
    io::stdin().read_line(&mut key)
        .expect("Failed to read key");
    // Convert to u8
    let key: u8 = key.trim().parse().expect("Invalid Input");

    let mut file_name = String::new();
    println!("Enter filename:");
    io::stdin().read_line(&mut file_name)
        .expect("Plain Text");

    let file_name = file_name.trim();

    use std::fs::File;
    let mut f = File::open(file_name).expect("Unable to open");
    let mut cipher_text = String::new();
    f.read_to_string(&mut cipher_text).unwrap();

    print!("Cipher text: \n{}", cipher_text);

    print!("Plain Text:\n{}", caesar::decipher(&cipher_text, key));
}

