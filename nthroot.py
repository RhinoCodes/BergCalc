from decimal import Decimal, getcontext, ROUND_HALF_UP
from math import sqrt
# Set precision higher than target to avoid cancellation errors
getcontext().prec = 32

# f(x) = n^2 + x = 0

def nthroot(x, n=2):
    guess = Decimal(x) / 2

    for i in range(7):
        correction = (guess ** n - x) / (n * (guess ** (n-1)))
        guess -= correction
    
    return guess