pub fn encrypt(plain_text: &String, depth: &usize) -> String {
    /* TODO - Find a math formula if possible */

    let mut fence: Vec<Vec<char>> = init_fence(depth);

    let mut i = 0;
    let mut going_down = true;
    for c in plain_text.to_uppercase().chars() {
        if c < 'A' || c > 'Z' {
            continue;
        }

        fence[i].push(c);

        if (i == 0 && !going_down) || (i == depth-1 && going_down) {
            going_down = !going_down;
        }

        if going_down {
            i += 1;
        } else {
            i -= 1;
        }
    }

    let mut cipher_text = String::new();
    for rail in fence {
        for c in rail {
            cipher_text.push(c);
        }
    }
    cipher_text
}

pub fn decrypt(cipher_text: &String, depth: &usize) -> String {
    let cipher_vec: Vec<char> = cipher_text.chars().collect();
    let mut rail_fence = vec![vec!['X'; cipher_vec.len()]; *depth];

    /* Unroll the cipher text */
    let mut l = 0;
    for i in 0..rail_fence.len() {
        let mut k = 0;
        let mut going_down = true;
        for j in 0..cipher_vec.len() {
            if k == i {
                rail_fence[i][j] = cipher_vec[l];
                l += 1;
            }

            if (k == 0 && !going_down) || (k == depth - 1 && going_down) {
                going_down = !going_down;
            }

            if going_down {
                k += 1;
            } else {
                k -= 1;
            }
        }
    }

    /* Collect the plain text */
    let mut plain_text = String::new();
    let mut k = 0;
    let mut going_down = true;
    for j in 0..cipher_vec.len() {
        plain_text.push(rail_fence[k][j]);

        if (k == 0 && !going_down) || (k == depth - 1 && going_down) {
            going_down = !going_down;
        }

        if going_down {
            k += 1;
        } else {
            k -= 1;
        }
    }
    plain_text
}

fn init_fence(depth: &usize) -> Vec<Vec<char>> {
    let mut fence: Vec<Vec<char>> = Vec::new();

    for i in 0..*depth {
        let mut tmp: Vec<char> = Vec::new();
        fence.push(tmp);
    }

    fence
}
