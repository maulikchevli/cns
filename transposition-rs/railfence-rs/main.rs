use std::io;
use std::io::prelude::*;

mod rail_fence;

fn main() {
    let mut choice = String::new();

    loop {
        println!("Rail Fence cipher: \nencrypt, decrypt, exit");

        println!("Enter choice: ");
        io::stdin().read_line(&mut choice)
            .expect("Could not read choice");

        match choice.trim() {
            "encrypt" => encrypt_rail_fence(),
            "decrypt" => decrypt_rail_fence(),
            "exit" => break,
            _ => println!("Invalid Choice"),
        }

        choice.clear();
        println!("\n===========================\n");
    }
}

fn encrypt_rail_fence() {
    let mut file_name = String::new();
    println!("Enter file name: ");
    io::stdin().read_line(&mut file_name)
        .expect("Could not read file name");
    let file_name = file_name.trim();

    use std::fs::File;
    let mut f = File::open(file_name)
        .expect("Could not open file");
    let mut plain_text = String::new();
    f.read_to_string(&mut plain_text).unwrap();
    let plain_text = plain_text.trim().to_string();

    let mut depth = String::new();
    println!("Enter depth of Rail Fence: ");
    io::stdin().read_line(&mut depth)
        .expect("Could not read depth of rail fence");
    let depth: usize = depth.trim().parse().unwrap();

    println!("Plain Text: {}", plain_text);
    println!("Depth of rail fence: {}", depth);
    println!("Cipher Text: {}", rail_fence::encrypt(&plain_text, &depth));
}

fn decrypt_rail_fence() {
    let mut file_name = String::new();
    println!("Enter file name: ");
    io::stdin().read_line(&mut file_name)
        .expect("Could not read file name");
    let file_name = file_name.trim();

    use std::fs::File;
    let mut f = File::open(file_name)
        .expect("Could not open file");
    let mut cipher_text = String::new();
    f.read_to_string(&mut cipher_text).unwrap();
    let cipher_text = cipher_text.trim().to_string();

    let mut depth = String::new();
    println!("Enter depth of Rail Fence: ");
    io::stdin().read_line(&mut depth)
        .expect("Could not read depth of rail fence");
    let depth: usize = depth.trim().parse().unwrap();

    println!("Cipher Text: {}", cipher_text);
    println!("Depth of rail fence: {}", depth);
    println!("Plain Text: {}", rail_fence::decrypt(&cipher_text, &depth));
}
