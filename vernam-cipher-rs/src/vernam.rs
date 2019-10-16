use std::fs::File;
use std::io::prelude::*;

pub fn cipher(file_name: &str, key_file: &str, output_file: &str) {
    let mut file = File::open(file_name)
        .expect("Could not open file");

    let mut plain_text = String::new();
    file.read_to_string(&mut plain_text)
        .unwrap();

    println!("Keyfile: {}", key_file);
    let mut key_file = File::open(key_file)
        .expect("Could not open key file");
    /*
    let mut key = String::new();
    key_file.read_to_string(&mut key)
        .unwrap();
    */

    let mut key: Vec<u8> = Vec::new();
    key_file.read_to_end(&mut key)
        .expect("could not read file");
    println!("{:?}", key);

    let key_len = key.len();

    let mut cipher_vec = String::new();
    let mut i = 0;
    for c in plain_text.chars() {
        cipher_vec.push(((c as u8) ^ key[i % key_len]) as char);
        i += 1;
    }

    let mut f = File::create(output_file)
        .expect("Could Not create file");
    let cipher_vec = cipher_vec.to_string();
    f.write_all(cipher_vec.as_bytes());
}

pub fn decipher(file_name: &str, key_file: &str, output_file: &str) {
    cipher(file_name, key_file, output_file);
    std::fs::remove_file(key_file);
}
