pub fn encrypt(plain_text: &String, key: &String) -> String {
    let table: [[char; 5]; 5] = generate_table(key, true);

    for i in 0..5 {
        for j in 0..5 {
            print!("{} ", table[i][j]);
        }
        println!();
    }

    let paired_plain_text:Vec<char> = generate_paired_text(plain_text);

    let mut cipher_text = String::new();

    for i in 0..paired_plain_text.len() {

        // Skip two
        if i % 2 != 0 {
            continue
        }

        let p1 = find_position(paired_plain_text[i], &table);
        let p2 = find_position(paired_plain_text[i+1], &table);

        let (c1, c2) = find_substitute_position_enc(p1, p2);

        cipher_text.push(table[c1.0][c1.1]);
        cipher_text.push(table[c2.0][c2.1]);
    }

    println!("Paired Plain Text: ");
    for x in paired_plain_text {
        print!("{}", x);
    }
    println!();

    cipher_text
}

pub fn decrypt(cipher_text: &String, key: &String) -> String {
    if cipher_text.len() % 2 ==  1 {
        println!("Cipher of odd length is not possible!");
        return "error".to_string();
    }

    let table: [[char; 5]; 5] = generate_table(key, true);
    let mut paired_cipher_text: Vec<char> = Vec::new();
    for x in cipher_text.to_uppercase().chars() {
        if x < 'A' || x > 'Z' {
            continue;
        }
        paired_cipher_text.push(x);
    }

    let mut plain_text = String::new();

    for i in 0..paired_cipher_text.len() {
        // Skip two
        if i % 2 != 0 {
            continue;
        }

        let c1 = find_position(paired_cipher_text[i], &table);
        let c2 = find_position(paired_cipher_text[i+1], &table);

        let (p1, p2) = find_substitute_position_dec(c1, c2);

        plain_text.push(table[p1.0][p1.1]);
        plain_text.push(table[p2.0][p2.1]);
    }

    plain_text
}

fn generate_table(key: &String, replace_i_to_j: bool) -> [[char; 5]; 5] {
    let mut table_entries = key.to_uppercase() + "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    if replace_i_to_j {
        table_entries = table_entries.replace("J", "I");
    } else {
        table_entries = table_entries.replace("Q", "");
    }

    let mut char_entered = [false; 26];
    let mut table = [['A'; 5]; 5];

    let mut i = 0;
    let mut j = 0;

    for c in table_entries.chars() {
        if char_entered[c as usize - 'A' as usize] == true {
        } else {
            table[i][j] = c;
            j = (j+1)%5;
            if j == 0 {
                i = (i + 1) % 5;
            }

            char_entered[c as usize - 'A' as usize] = true;
        }
    }
    table
}

fn generate_paired_text(plain_text: &String) -> Vec<char> {
    let mut paired_plain_text:Vec<char> = Vec::new();
    let mut previous_char: char = ' ';
    let mut change = 0;
    for c in plain_text.to_uppercase().chars() {
        if c < 'A' || c > 'Z' {
            continue;
        }

        if c == previous_char {
            if change == 1 {
                paired_plain_text.push('X');
            }
        }
        paired_plain_text.push(c);
        previous_char = c;
        change = (change + 1) % 2;
    }

    if paired_plain_text.len() % 2 == 1 {
        paired_plain_text.push('X');
    }

    paired_plain_text
}

fn find_position(char_to_find: char, table: &[[char; 5]; 5]) -> (usize, usize) {
    for (i, row) in table.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == char_to_find {
                return (i, j);
            }
        }
    }

    (6, 6)
}

fn find_substitute_position_enc(p1: (usize, usize), p2: (usize, usize)) -> ((usize, usize), (usize, usize)) {
    let mut c1: (usize, usize) = p1;
    let mut c2: (usize, usize) = p2;

    // If Letters are of same column
    if p1.1 == p2.1 {
        c1.0 = (p1.0 + 1) % 5;
        c1.1 = p1.1;

        c2.0 = (p2.0 + 1) % 5;
        c2.1 = p2.1;
    }
    // Letters are of same row
    else if p1.0 == p2.0 {
        c1.0 = p1.0;
        c1.1 = (p1.1 + 1) % 5;

        c2.0 = p2.0;
        c2.1 = (p2.1 + 1) % 5;
    }
    else {
        c1.0 = p1.0;
        c1.1 = p2.1;

        c2.0 = p2.0;
        c2.1 = p1.1;
    }

    (c1, c2)
}

fn find_substitute_position_dec(p1: (usize, usize), p2: (usize, usize)) -> ((usize, usize), (usize, usize)) {
    let mut c1: (usize, usize) = p1;
    let mut c2: (usize, usize) = p2;

    // If Letters are of same column
    if p1.1 == p2.1 {
        c1.0 = (p1.0 + 4) % 5;
        c1.1 = p1.1;

        c2.0 = (p2.0 + 4) % 5;
        c2.1 = p2.1;
    }
    // Letters are of same row
    else if p1.0 == p2.0 {
        c1.0 = p1.0;
        c1.1 = (p1.1 + 4) % 5;

        c2.0 = p2.0;
        c2.1 = (p2.1 + 4) % 5;
    }
    else {
        c1.0 = p1.0;
        c1.1 = p2.1;

        c2.0 = p2.0;
        c2.1 = p1.1;
    }

    (c1, c2)
}
