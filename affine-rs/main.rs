use std::io;
use std::io::Read;

mod affine;

fn main() {
    let mut choice = String::new();
    loop {
        println!("AFFINE CIPHER.");
        println!("encrypt, decrypt, exit");
        println!("Enter Choice.");

        io::stdin().read_line(&mut choice)
            .expect("Could not read choice.");

        match choice.trim() {
            "encrypt" => encrypt_affine(),
            "decrypt" => decrypt_affine(),
            "exit" => break,
            _ => println!("Invalid Choice"),
        }

        choice.clear();

        println!("==========================");
    }
}

fn encrypt_affine() {
    let mut file_name = String::new();
    println!("Enter file name.");
    io::stdin().read_line(&mut file_name)
        .expect("Could not read filename");
    let file_name = file_name.trim();

    use std::fs::File;
    let mut f = File::open(file_name)
        .expect("Could Not open file");

    let mut plain_text = String::new();
    f.read_to_string(&mut plain_text)
        .expect("Could Not read plain text from file");
    let plain_text = plain_text.trim().to_string();

    // Take 'a' in ax + b
    let mut a = String::new();
    println!("Enter multiplier.");
    io::stdin().read_line(&mut a)
        .expect("Could not read 'a'");

    // Take 'b' in ax + b
    let mut b = String::new();
    println!("Enter Addition.");
    io::stdin().read_line(&mut b)
        .expect("Could not read 'b'");

    // Type cast a and b
    let a: usize = a.trim().parse()
        .expect("Bad Input for 'a'");
    let b: usize = b.trim().parse()
        .expect("Bad INput for 'b'");

    // Print results
    println!("Plain Text: {}", plain_text);
    println!("a: {}, b: {}", a, b);
    println!("Cipher Text: {}", affine::encrypt(&plain_text, a, b));
}

fn decrypt_affine() {
    let mut file_name = String::new();
    println!("Enter file name.");
    io::stdin().read_line(&mut file_name)
        .expect("Could not read filename");
    let file_name = file_name.trim();

    use std::fs::File;
    let mut f = File::open(file_name)
        .expect("Could Not open file");

    let mut cipher_text = String::new();
    f.read_to_string(&mut cipher_text)
        .expect("Could Not read plain text from file");
    let plain_text = cipher_text.trim().to_string();

    // Take 'a' in ax + b
    let mut a = String::new();
    println!("Enter multiplier.");
    io::stdin().read_line(&mut a)
        .expect("Could not read 'a'");

    // Take 'b' in ax + b
    let mut b = String::new();
    println!("Enter Addition.");
    io::stdin().read_line(&mut b)
        .expect("Could not read 'b'");

    // Type cast a and b
    let a: usize = a.trim().parse()
        .expect("Bad Input for 'a'");
    let b: usize = b.trim().parse()
        .expect("Bad INput for 'b'");

    // Print results
    println!("Cipher Text: {}", plain_text);
    println!("a: {}, b: {}", a, b);
    println!("Plain Text: {}", affine::decrypt(&cipher_text, a, b));
}
