use std::io;
use std::io::prelude::*;
use std::fs::File;
use rand::Rng;

mod vernam;

fn main() {
    let mut choice = String::new();
    loop {
        println!("VERNAM CIPHER.");
        println!("encrypt, decrypt, exit");
        println!("Enter Choice.");

        io::stdin().read_line(&mut choice)
            .expect("Could not read choice.");

        match choice.trim() {
            "encrypt" => encrypt_vernam(),
            "decrypt" => decrypt_vernam(),
            "exit" => break,
            _ => println!("Invalid Choice"),
        }

        choice.clear();

        println!("==========================");
    }
}

fn generate_key(key_len: usize) {
    let mut file = File::create("key.txt")
        .expect("Could Not create file");

    let mut key = String::new();

    for i in 0..key_len {
        let rand_num: u8 = rand::thread_rng().gen_range(0, 127);
        println!("rand_num {} char {}", rand_num, rand_num as char);
        key.push(rand_num as char);
    }

    file.write_all(key.as_bytes());
}

fn encrypt_vernam() {
    let key_len = 20;
    generate_key(key_len);

    let mut file_name = String::new();
    println!("Enter plain text file name");
    io::stdin().read_line(&mut file_name)
        .expect("Could not read file name");
    let file_name = file_name.trim();

    let key_file = "key.txt";
    let output_file = "enc.txt";
    vernam::cipher(file_name, key_file, output_file);
}

fn decrypt_vernam() {
    let mut file_name = String::new();
    println!("Enter cipher text file name");
    io::stdin().read_line(&mut file_name)
        .expect("Could not read file name");
    let file_name = file_name.trim();

    let key_file = "key.txt";
    let output_file = "dec.txt";
    vernam::decipher(file_name, key_file, output_file);
}
