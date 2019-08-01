/* CaesarCipher
 * 
 * input: plaintext, key
 * output: ciphertext
 */
char * CaesarCipher(char * plaintext, int key)
{
	int len_plaintext = strlen(plaintext);
	char * ciphertext = malloc( sizeof(char) * len_plaintext);

	int i;
	for(i = 0; plaintext[i] != '\0'; i++) {
		char char_int_bridge;
		if(plaintext[i] >= 'a' && plaintext[i] <= 'z') {
			char_int_bridge = 'a';
		} else if(plaintext[i] >= 'A' && plaintext[i] <= 'Z') {
			char_int_bridge = 'A';
		} else {
			ciphertext[i] = plaintext[i];
			continue;
		}

		ciphertext[i] = ((plaintext[i] - char_int_bridge + key) % 26) + char_int_bridge;
	}

	ciphertext[i++] = '\0';

	return ciphertext;
}

