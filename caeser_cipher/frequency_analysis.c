#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "caesar_cipher.c"
#include "character_frequency.c"

int main(int argc, char * argv[])
{
	int key;
	const int MAX_SIZE = 100;
	char ciphertext[MAX_SIZE];
	char * plaintext = NULL;

	printf("Enter ciphertext: ");
	scanf(" %[^\n]s", ciphertext);
	
	char sorted_frequency_table[] = {'e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd',
		'l', 'c', 'u', 'm'. 'w', 'f', 'g', 'y', 'p', 'b', 'v', 'k', 'j', 'x',
		'q', 'z'};
	
	plaintext = ReplaceByFrequency( ciphertext, sorted_frequency_table);
	return 0;
}

char * ReplaceByFrequency(char * ciphertext, char * frequency_table)
{
	int len = strlen(ciphertext);
	char * plaintext = (char *) malloc(len * sizeof(char));

	char char_frequency[26] = {0};
	FindCharacterFreq( ciphertext, char_frequency);

	for(int i=0; ciphertext[i] != '\0'; i++) {
		int c = ciphertext[i] - 'a';
	}
}

