#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "caesar_cipher.c"

int main() 
{

	int key;
	const int MAX_SIZE = 100;
	char plaintext[MAX_SIZE];
	char * ciphertext;

	printf("Enter plaintext: ");
	scanf(" %[^\n]s", plaintext);

	ciphertext = CaesarCipher(plaintext, 3);

	printf("\n%s", ciphertext);

	return 0;
}

