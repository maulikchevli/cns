char_in_table = [False]*26
playfair = []

def padded_par(plain_text):
	new_text = ""
	j = 1
	while j < len(plain_text):
		c = plain_text[j]
		if c == plain_text[j-1]:
			new_text += plain_text[j-1] + 'x'
			j += 1
		else:
			new_text += plain_text[j-1] + c
			j += 2
	
	return new_text

def find_in_matrix(a):
	for i in range(5):
		for j in range(5):
			if playfair[i][j] == a:
				return (i,j)


def encrypt(plain_text):
	pp = padded_par(plain_text)
	cipher_text = ""
	i = 1
	while i < len(pp):
		a = pp[i-1]
		b = pp[i]
		i += 2

		ai, aj = find_in_matrix(a)
		bi, bj = find_in_matrix(b)

		if ai == bi:
			aj = (aj+1)%5
			bj = (bj + 1)%5
		elif aj == bj:
			ai = (ai+1)%5
			bi = (bi+1)%5
		else:
			aj,bj = bj,aj
		cipher_text += playfair[ai][aj] + playfair[bi][bj]

	print(cipher_text)
	pass

def decrypt(cipher_text):
	pp = padded_par(cipher_text)
	plain_text = ""
	i = 1
	while i < len(pp):
		a = pp[i-1]
		b = pp[i]
		i += 2

		ai, aj = find_in_matrix(a)
		bi, bj = find_in_matrix(b)

		if ai == bi:
			aj = (aj-1)%5
			bj = (bj - 1)%5
		elif aj == bj:
			ai = (ai-1)%5
			bi = (bi-1)%5
		else:
			aj,bj = bj,aj
		plain_text += playfair[ai][aj] + playfair[bi][bj]
	print(plain_text)

def generate_table():
	tmp = []
	key = input('Enter Playfair key: ')
	
	i = 0
	j = 0
	for c in key:
		if char_in_table[ord(c) - ord('a')] == False:
			#print(c)
			tmp.append(c)
			char_in_table[ord(c) - ord('a')] = True
			j += 1

		if j == 5:
			j = 0
			i += 1
			playfair.append(tmp)
			tmp = []
	
	for k in range(26):
		if char_in_table[k] == False:
			if chr(k+ord('a')) != 'j':
				#print(chr(k + ord('a')))
				tmp.append(chr(k+ord('a')))
				char_in_table[k] = True
				j += 1

		if j == 5:
			j = 0
			i += 1
			playfair.append(tmp)
			tmp = []

	pass

def show_table():
	for i in range(5):
		print(playfair[i])

def main():
	generate_table()
	show_table()

if __name__ == "__main__":
	main()

