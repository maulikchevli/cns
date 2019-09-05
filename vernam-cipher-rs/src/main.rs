use std::io;
use rand::Rng;

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

fn encrypt_vernam() {
    let mut rand_num = rand::thread_rng().gen_range(1, 101);
    println!("Random number: {}", rand_num);

    let mut file_name = String::new();
    println!("Enter plain text file name");
    io::stdin().read_line(&mut file_name)
        .expect("Could not read file name");
    let file_name = file_name.trim();

    use std::fs::File;
    let mut file = File::open(file_name)
        .expect("Could not open file");
}

fn decrypt_vernam() {
    println!("Decrypt");
}
