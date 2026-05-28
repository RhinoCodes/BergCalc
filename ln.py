from decimal import Decimal, getcontext, ROUND_HALF_UP
from trig import PI
# Set precision higher than target to avoid cancellation errors
getcontext().prec = 35
LN2 = Decimal("0.693147180559945309417232121458176568")

def agm(a, b, n=7):
    a = Decimal(a)
    b = Decimal(b)
    for i in range(n):
        a, b = (a + b) / 2, (a * b).sqrt()
    return (a + b) / 2

def ln(x, m=30):
    x = Decimal(x)
    s = x * (2 ** m)
    
    # Constants at full 30-digit precision
    
    result = PI / (2 * agm(1, Decimal(4) / s))

    result = (result - m * LN2)
    result = result.quantize(Decimal("10") ** -32, ROUND_HALF_UP)
    return result

def log(x, b=2):
    return ln(x) / ln(b)