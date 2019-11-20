import math

def main():
	pass

def gen_key_index(key):
	key = key.upper()
	key = [ord(k) for k in key]
	print(key)
	key_index = [0] * len(key)
	rank = 0
	for i in range(ord('A'), ord('Z')+1):
		for (index,j) in enumerate(key):
			if j == i:
				key_index[index] = rank
				rank += 1
	return key_index

def create_matrix(plain_text, key):
	key = key.upper()
	key = [ord(k) for k in key]
	print(key)
	key_index = [0] * len(key)
	rank = 0
	for i in range(ord('A'), ord('Z')+1):
		for (index,j) in enumerate(key):
			if j == i:
				key_index[index] = rank
				rank += 1
	
	matrix = []
	cols = len(key)
	tmp = []
	for c in plain_text:
		tmp.append(c)
		to_append = True
		if len(tmp) == cols:
			matrix.append(tmp)
			to_append = False
			tmp = []
	if to_append:
		matrix.append(tmp)
	return matrix, key_index

def encrypt(plain_text, key):
	plain_text = plain_text.upper()
	matrix, key_index = create_matrix(plain_text, key)
	print(matrix)

	cipher_text = ""
	for Num in range(len(key_index)):
		for (idx, val) in enumerate(key_index):
			if Num == val:
				for i in range(len(matrix)):
					if idx < len(matrix[i]):
						print(matrix[i][idx])
						cipher_text += matrix[i][idx]

	print(cipher_text)
	

def decrypt(cipher_text, key):
	cipher_text = cipher_text.upper()
	key_index = gen_key_index(key)
	rows = math.ceil(len(cipher_text) / len(key))
	len_last_row = len(cipher_text) - len(key)*(rows-1)

	matrix = [[""] * len(key) for _ in range(rows)]

	
	return matrix

if __name__ == '__main__':
	main()

