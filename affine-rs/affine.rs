pub fn encrypt(plain_text: &String, a: usize, b: usize) -> String {
    let mut cipher_text = String::new();
    let modulo = 26;

    let mut enc_ch: char;
    for c in plain_text.to_uppercase().chars() {
        if c < 'A' || c > 'Z' {
            continue;
        }

        let t1 = (a * (c as usize - 'A' as usize)) % modulo;
        let t2 = (t1 + b) % modulo + 'A' as usize;
        enc_ch = t2 as u8 as char;
        cipher_text.push(enc_ch);
    }

    cipher_text
}

pub fn decrypt(cipher_text: &String, a: usize, b:usize) -> String {
    let modulo: isize = 26;

    let (_, _, a_inv) = e_gcd(modulo, a as isize);
    let a_inv: usize = if a_inv < 0 {
        (a_inv + modulo) as usize
    } else {
        a_inv as usize
    };

    let mut plain_text = String::new();
    let mut dec_ch: char;
    for c in cipher_text.to_uppercase().chars() {
        if c < 'A' || c > 'Z' {
            continue;
        }

        let mut t3 = ((c as usize - 'A' as usize) as isize - b as isize ) % modulo;
        if t3 < 0 {
            t3 += modulo;
        }

        let t1 = t3 as usize;
        let t2 = (t1 * a_inv) % modulo as usize + 'A' as usize;

        dec_ch = t2 as u8 as char;
        plain_text.push(dec_ch);
    }

    plain_text
}

fn e_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
        (b, 0, 1)
    }
    else {
        let (t, x, y) = e_gcd(b % a, a);
        (t, y - (b / a)* x, x)
    }
}

