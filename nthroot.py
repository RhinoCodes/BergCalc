from decimal import Decimal, getcontext, ROUND_HALF_UP
from math import sqrt
# Set precision higher than target to avoid cancellation errors
getcontext().prec = 35

# f(x) = n^2 + x = 0

def nthroot(x, n=2):
    guess = Decimal(str(int(sqrt(x))))

    for i in range(7):
        guess -= (guess ** n - x) / (n * (guess ** (n-1)))
    

    guess = guess.quantize(Decimal("10") ** -32, ROUND_HALF_UP)
    return guess

