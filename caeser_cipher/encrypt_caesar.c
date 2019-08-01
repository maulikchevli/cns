#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "caesar_cipher.c"

int main(int argc, char * argv[])
{
	int key;
	const int MAX_SIZE = 100;
	char plaintext[MAX_SIZE];
	char * ciphertext = NULL;

	printf("Enter plaintext: ");
	scanf(" %[^\n]s", plaintext);

	printf("Enter Key:");
	scanf("%d", &key);

	ciphertext = CaesarCipher(plaintext, key);

	printf("Cipher text: %s \n", ciphertext);
	return 0;
}

