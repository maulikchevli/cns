def encrypt(plain_text, key):
	cipher_text = ""
	for c in plain_text:
		cipher_text += chr((ord(c) - ord('a') + key)%26 + ord('a'))
	
	print(cipher_text)

def decrypt(cipher_text, key):
	encrypt(cipher_text, 26-key)

def main():
	encrypt("hello world", 1)

if __name__ == "__main__":
	main()
