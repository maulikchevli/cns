def mod_inv(mod, a):
	g, x, y = e_gcd(a, mod)
	if g != 1:
		return 0
	else:
		return x % mod

def e_gcd(a, b):
	if a == 0:
		return (b, 0, 1)
	else:
		g, y, x = e_gcd(b % a, a)
		return (g, x - (b // a)*y, y)

