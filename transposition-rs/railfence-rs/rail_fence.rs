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
    let mut plain_text = String::from("Plain");

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
