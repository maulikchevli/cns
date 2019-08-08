pub fn encrypt(plain_text: &String, key: &String) -> String {
	  let table = generate_table(key, true);

    for i in 0..5 {
        for j in 0..5 {
            print!("{} ", table[i][j]);
        }
        println!();
    }

    let paired_plain_text:Vec<char> = generate_paired_text(plain_text);

    /*
    for i in 0..paired_plain_text.len() {
    (r1, c1) = find_position(paired_plain_text[i], &table);
    (r2, c2) = find_position(paired_plain_text[i+1], &table);
}
     */
	  paired_plain_text.into_iter().collect()
}

fn generate_table(key: &String, replaceItoJ: bool) -> [[char; 5]; 5] {
	  let mut table_entries = key.to_uppercase() + "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    if replaceItoJ {
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
    for c in plain_text.to_uppercase().chars() {
        if c <= 'A' || c >= 'Z' {
            continue;
        }

        if c == previous_char {
            paired_plain_text.push('X');
        }
        paired_plain_text.push(c);
        previous_char = c;
    }

    if paired_plain_text.len() % 2 == 1 {
        paired_plain_text.push('X');
    }

    paired_plain_text
}

