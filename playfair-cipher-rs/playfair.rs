pub fn encrypt(plain_text: &String, key: &String) -> String {
	generate_table(key);
	let mut cipher_text = String::from("Hello");
	cipher_text
}

fn generate_table(key: &String) {
	let mut table_entries = key.to_uppercase() + "ABCDEFGHIJKLMNOPQRST";
	println!("table {}", table_entries);

	let mut char_entered = [false; 26];
	let mut table = [[0u8; 5]; 5];

	let mut i = 0;
	let mut j = 0;

	print!("{} ", char_entered[6]);
	/*
	for c in table_entries.chars() {
		if char_entered[c as u32 - 'A' as u32] == true {
		} else {
			table[i][j] = c;
			j = (j+1)%5;
			if j == 0 {
				i = (i + 1) % 5;
			}
		}
	}
	*/
}

