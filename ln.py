from decimal import Decimal, getcontext, ROUND_HALF_UP
from trig import PI
# Set precision higher than target to avoid cancellation errors
getcontext().prec = 35
LN2 = Decimal("0.6931471805599453094172321214581765680755001343602552541206800094933936219696947156058633269964186875")

def agm(a, b, n=12):
    a = Decimal(str(a))
    b = Decimal(str(b))
    for i in range(n):
        a, b = (a + b) / 2, (a * b).sqrt()
    return (a + b) / 2

def ln(x, m=60):
    x = Decimal(str(x))
    s = x * (Decimal("2") ** m)
    if s == 0:
        return None
    # Constants at full 30-digit precision
    
    result = PI / (2 * agm(1, Decimal(4) / s))
    result = (result - m * LN2)
    return result

def log(x, b=2):
    return ln(x) / ln(b)