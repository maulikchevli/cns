#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "caesar_cipher.c"

int Key_from_pair(char * ciphertext, char * plaintext);

int main(int argc, char * argv[])
{
	int key;
	const int MAX_SIZE = 100;
	char plaintext[MAX_SIZE];
	char ciphertext[MAX_SIZE];

	printf("Enter plaintext: ");
	scanf(" %[^\n]s", plaintext);

	printf("Enter ciphertext: ");
	scanf(" %[^\n]s", ciphertext);

	printf("Key: %d\n", Key_from_pair(ciphertext, plaintext));

	return 0;
}

int Key_from_pair(char * ciphertext, char * plaintext)
{
	int key = abs(ciphertext[0] - plaintext[0]);
	char * computed_caeser = CaesarCipher(plaintext, key);

	if(strcmp(computed_caeser, ciphertext) == 0) {
		return key;
	} else {
		return -99;
	}
}

