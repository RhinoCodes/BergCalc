from decimal import Decimal, getcontext, ROUND_HALF_UP
from math import sqrt
from utils.ln import ln, LN2

getcontext().prec = 35

def exp(x):
    x = Decimal(str(x))
    if x < 0:
        return 1 / exp(-x)

    m = int(x / ln(2))
    remainder = x - m * LN2
    approx = 1
    last = 1

    for i in range(1,50):
        last = last * remainder / i
        approx += last

        if last < Decimal("1e-40"):
            break

    approx = (approx * (2**m))
    #approx = approx.quantize(Decimal("10") ** -28, ROUND_HALF_UP)
    return approx