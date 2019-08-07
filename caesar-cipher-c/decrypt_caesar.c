#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "caesar_cipher.c"

char * invCaesarCipher(char * ciphertext, int key);
int main(int argc, char * argv[])
{
	int key;
	const int MAX_SIZE = 100;
	char ciphertext[MAX_SIZE];
	char * plaintext = NULL;

	printf("Enter ciphertext: ");
	scanf(" %[^\n]s", ciphertext);

	printf("Enter Key:");
	scanf("%d", &key);

	plaintext = invCaesarCipher(ciphertext, key);

	printf("Cipher text: %s \n", plaintext);
	return 0;
}

char * invCaesarCipher(char * ciphertext, int key)
{
	return CaesarCipher(ciphertext, 26 - key);
}

