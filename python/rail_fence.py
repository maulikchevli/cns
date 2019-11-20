def main():
	pass

def encrypt(plain_text, depth):
	plain_text = plain_text.replace(" ", "")
	plain_text = plain_text.upper()

	rails = [""] * depth
	d = 0
	to_fro_ptr = 0
	going_down = True
	for c in plain_text:
		rails[to_fro_ptr] += c
		if (to_fro_ptr == 0 and not going_down) or (to_fro_ptr == depth -1 and going_down):
			going_down = not going_down

		to_fro_ptr += 1 if going_down else -1
	
	print(rails)
	cipher_text = "".join(rails)
	return cipher_text

def decrypt(cipher_text, depth):
	cipher_text = cipher_text.replace(" ", "")
	cipher_text = cipher_text.upper()

	rails = [[""] * len(cipher_text) for _ in range(depth)]
	text_ptr = 0
	for i in range(len(rails)):
		to_fro_ptr = 0
		going_down = True
		for j in range(len(cipher_text)):
			if to_fro_ptr == i:
				rails[i][j] = cipher_text[text_ptr]
				text_ptr += 1
			if (to_fro_ptr == 0 and not going_down) or (to_fro_ptr == depth -1 and going_down):
				going_down = not going_down

			to_fro_ptr += 1 if going_down else -1
	
	to_fro_ptr = 0
	going_down = True
	plain_text = ""
	for j in range(len(cipher_text)):
		#print(to_fro_ptr, j)
		plain_text += rails[to_fro_ptr][j]			
		if (to_fro_ptr == 0 and not going_down) or (to_fro_ptr == depth -1 and going_down):
			going_down = not going_down

		to_fro_ptr += 1 if going_down else -1
	
	return plain_text

if __name__ == "__main__":
	main()
