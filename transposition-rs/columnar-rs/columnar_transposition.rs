pub fn encrypt(plain_text: &String, key: &String) -> String {
    let mut cipher_text = String::from(plain_text);

    let key_len = key.len();

    let col_pbox: Vec<usize> = pbox_from_key(key);

    let plain_vec: Vec<Vec<char>> = generate_vec(plain_text, key_len);
    for row in plain_vec.iter() {
        for x in row.iter() {
            print!("{} ", x);
        }
        println!();
    }

    let mut pbox: Vec<(usize, usize)> = Vec::new();
    for (i, x) in col_pbox.iter().enumerate() {
        pbox.push((i, *x));
    }
    pbox.sort_by_key(|a| a.1);

    /* Applying Permutation */
    let group_len = 5;
    let mut space_cnt = 0;
    for (col, _) in pbox {
        for row in &plain_vec {
            if col >= row.len() {
                continue;
            }

            cipher_text.push(row[col]);

            space_cnt += 1;
            if space_cnt == group_len {
                space_cnt = 0;
                cipher_text.push(' ');
            }
        }
    }

    cipher_text
}

pub fn decrypt(cipher_text: &String, key: &String) -> String {
    let mut plain_text = String::from(cipher_text);

    plain_text
}

fn generate_vec(plain_text: &String, key_len: usize) -> Vec<Vec<char>> {
    let mut plain_vec: Vec<Vec<char>> = Vec::new();
    let mut k = key_len;

    let mut tmp: Vec<char> = Vec::new();
    for c in plain_text.chars() {
        if (c < 'a' || c > 'z') && (c < 'A' || c > 'Z') {
            continue;
        }

        if k == 0 {
            k = key_len;
            plain_vec.push(tmp);
            tmp = Vec::new();
        }
        tmp.push(c);
        k -= 1;
    }
    // Push the last tmp vec
    plain_vec.push(tmp);

    plain_vec
}

fn pbox_from_key(key: &String) -> Vec<usize> {
    let mut pbox: Vec<usize> = Vec::new();
    //let mut key_vec: Vec<char> = key.chars().collect();

    for (i, c) in key.chars().enumerate() {
        let mut char_val: usize = 0;
        for (j, k) in key.chars().enumerate() {
            if c > k {
                char_val += 1;
            }
            else if c == k && i > j {
                char_val += 1;
            }
        }
        pbox.push(char_val);
    }
    pbox
}
