void FindCharacterFreq(char * text, char * frequency_array)
{
	char c;
	for(int i = 0; text[i] != '\0'; i++) {
		if(text[i] >= 'A' && text[i] <= 'Z') {
			c = text[i] - 'A';	
		} else if(text[i] >= 'a' && text[i] <= 'z') {
			c = text[i] - 'a';	
		} else {
			continue;
		}

		frequency_array[c]++;
	}
	return;
}
