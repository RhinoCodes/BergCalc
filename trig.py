from decimal import Decimal, getcontext, ROUND_HALF_UP
from math import sqrt, floor
from math import asin as float_arcsin

getcontext().prec = 35
PI  = Decimal("3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679")
def sin(x):
    x = Decimal(str(x))

    if (x < 0):
        return -sin(-x)

    x = x - (floor(x / (2*PI)) * 2*PI) # x = x mod 2pi
    negate = False
    if (x > PI):
        negate = True
        x = x - PI

    if (x > PI/4):
        approx = cos (PI/2 - x)
        return (-approx if negate else approx)
    
    approx = x
    last = x
    for n in range(3,80,2):
        last = -last * (x**2)/(n * (n-1))
        approx += last
        if abs(last) < Decimal("1e-32"):
            break

    approx = approx.quantize(Decimal("10") ** -32, ROUND_HALF_UP)

    return (-approx if negate else approx)

def cos(x):
    x = Decimal(str(x))

    if x < 0:
        return cos(-x)

    x = x - (floor(x / (2*PI)) * 2*PI)
    negate = False
    if (x > PI/2 and x < 3 * PI/2):
        negate = True
        x = abs(x - PI)

    if (x > PI/4):
        result = sin (PI/2 - x)
        return -result if negate else result
    
    approx = 1
    last = 1
    for n in range(2,200,2):
        last = -last * (x**2)/(n * (n-1))
        approx += last
        if abs(last) < Decimal("1e-32"):
            break

    return (-approx if negate else approx)

def tan(x):
    c = cos(x)
    if c == 0:
        raise ValueError("Tan undefined at this value")
    return sin(x) / c

def cot(x):
    s = sin(x)
    if s == 0:
        raise ValueError("Cot undefined at this value")

    return s / cos(x)

def sec(x):
    c = cos(x)
    if c == 0:
        raise ValueError("Sec undefined at this value")
    return 1 / c

def csc(x):
    s = sin(x)
    if s == 0:
        raise ValueError("Csc undefined at this value")
    return 1 / s

def arcsin(x):
    x = Decimal(str(x))
    if x == 1:
        return PI/2
    elif x == -1:
        return -PI/2
    if abs(x) > 1:
        raise ValueError("Inverse trig functions only defined on -1 to 1")
    if abs(x) > 0.9:
        inner = nthroot((1 - x) / 2)
        return PI/2 - 2 * arcsin(inner)

    guess = Decimal(str(float_arcsin(float(x))))
    for i in range(100):
        old = guess
        guess -= (sin(guess) - x) / (cos(guess))
        if abs(guess - old) < Decimal("1e-32"):
           break
    
    return guess

def arccos(x):
    return PI/2 - arcsin(x)