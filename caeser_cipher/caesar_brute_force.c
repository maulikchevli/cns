#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "caesar_cipher.c"

void Caesar_brute_force( char * ciphertext);

int main(int argc, char * argv[])
{
	int key;
	const int MAX_SIZE = 100;
	char plaintext[MAX_SIZE];
	char ciphertext[MAX_SIZE];

	printf("Enter ciphertext: ");
	scanf(" %[^\n]s", ciphertext);

	Caesar_brute_force(ciphertext);
	return 0;
}

void Caesar_brute_force(char * ciphertext)
{
	int key;
	char * computer_caeser = NULL;
	for(key = 0; key < 26; key++) {
		computer_caeser = CaesarCipher(ciphertext, 25-key);
		printf("key %d, cipher: %s\n", key + 1, computer_caeser);

		free(computer_caeser);
		computer_caeser = NULL;
	}
	return;
}

