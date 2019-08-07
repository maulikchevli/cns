use std::collections::HashMap;

pub fn cipher(plain_text: &String, key: u8) -> String {
    let mut cipher_text = String::with_capacity(plain_text.len());
    let mut m: u8;

    for c in plain_text.chars() {
        if c >= 'a' && c <= 'z' {
            let char_to_int = 'a';
            m = ((c as u8 - char_to_int as u8) + key) % 26 + char_to_int as u8;
        } else if c >= 'A' && c <= 'Z' {
            let char_to_int = 'A';
            m = ((c as u8 - char_to_int as u8) + key) % 26 + char_to_int as u8;
        } else {
            m =  c as u8;
        }
        cipher_text.push(m as char);
    }

    cipher_text
}

pub fn decipher(cipher_text: &String, key: u8) -> String {
    let number_of_alphabets = 26;
    cipher(&cipher_text, number_of_alphabets - key)
}

/* Frequency Analysis
 *
 * APPROXIMATE method
 *
 * returns: plain text
 */
pub fn frequency_analysis(cipher_text: &String) -> String {
    let cipher = cipher_text.trim();

    let mut calculated_freq: HashMap<char, u32> = HashMap::new();
    for c in cipher.to_lowercase().chars() {
        *calculated_freq.entry(c).or_insert(0) += 1;
    }

    let mut calculated_freq: Vec<_> = calculated_freq.iter().collect();
    calculated_freq.sort_by(|a, b| a.1.cmp(b.1).reverse());

    // Retain only alphabets and remove special chars
    calculated_freq.retain(|x| x.0 >= &'a' && x.0 <= &'z');

    let frequency_table: [char; 26] = ['e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'c', 'u', 'm', 'w', 'f', 'g', 'y', 'p', 'b', 'v', 'k', 'j', 'x', 'q', 'z'];

    let mut remap: HashMap<char, char> = HashMap::new();
    for i in 0..calculated_freq.len() {
        remap.insert(*calculated_freq[i].0, frequency_table[i]);
    }

    let mut plain_text = String::with_capacity(cipher_text.len());
    for x in cipher_text.to_lowercase().chars() {
        let m: char;

        if x >= 'a' && x <= 'z' {
            m = remap[&x];
        } else {
            m = x;
        }
        plain_text.push(m);
    }

    plain_text
}

