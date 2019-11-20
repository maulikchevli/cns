import binascii

n = 9516311845790656153499716760847001433441357    # p*q = modulus
e = 65537
d = 5617843187844953170308463622230283376298685

message = "hello world"
print("Plain Text:", message)

hex_msg = binascii.hexlify(message.encode())
print("Hex Data :", hex_msg)

plain_text = int(hex_msg, 16)
print("Plain text in int: ", plain_text)

if plain_text > n:
    raise Exception("Plain Text too large for key")

enc_text = pow(plain_text, e, n)
print("Encrypted Text:", enc_text)

dec_text = pow(enc_text, d, n)
print("Dec text: ", dec_text)

print("Dec Message: ", binascii.unhexlify(hex(dec_text)[2:]).decode())
