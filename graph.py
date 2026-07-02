import matplotlib.pyplot as plt
import numpy as np
from symbolicalgebra import Parser, tree 
from math import cos
from integration import integrate
from decimal import Decimal, getcontext, ROUND_HALF_UP
getcontext().prec = 35
def sample(f, x_min, x_max, width_px, dx):
    sx = Decimal(str(width_px)) / Decimal(str(x_max)) - Decimal(str(x_min))
    sy = sx = Decimal(str(370)) / Decimal(str(x_max)) - Decimal(str(x_min))
    xs = []
    ys = []
    x = Decimal(str(x_min))
    while x <= x_max:
        xs.append(x)
        try:
            y = f(x)
        except (ZeroDivisionError, ValueError):
            y = float('nan')
        ys.append(y)
        if callable(dx):
            dx = Decimal(1) / (sx**2 + (dx(x) * sy)**2).sqrt()
        else:
            dx = Decimal(1) / (sx**2 + (dx * sy)**2).sqrt()
        if dx < ((Decimal(str(x_max)) - Decimal(str(x_min))) / Decimal(str(width_px))) * Decimal("0.1"):
            dx = ((Decimal(str(x_max)) - Decimal(str(x_min))) / Decimal(str(width_px))) * Decimal("0.1")
        if dx > ((Decimal(str(x_max)) - Decimal(str(x_min))) / Decimal(str(width_px))) * Decimal("4"):
            dx = ((Decimal(str(x_max)) - Decimal(str(x_min))) / Decimal(str(width_px))) * Decimal("4")
        x += dx    
    return xs, ys

func_str = input("Function: ")
func = tree(Parser(func_str).firstPass())
my_function = lambda x: func.evaluate({"x": x}).termOne

derivative = lambda x: func.differentiate().evaluate({"x": x}).termOne;
xs, ys = sample(my_function, -10, 10, 496, derivative)
plt.scatter(xs, ys, s=0.1)
last_x = xs[0]
last_y = ys[0]
for x,y in zip(xs, ys):
    if last_y == None or y == None:
        last_x = x
        last_y = y
        continue
    if abs(last_y - y) > 1000:  # Skip large jumps to avoid false root detection
        last_x = x
        last_y = y
        continue
    if (last_y > 0 and y < 0 or last_y < 0 and y > 0) or (derivative(x) > 0 and derivative(last_x) < 0 or derivative(x) < 0 and derivative(last_x) > 0):
        guess = Decimal(str((x + last_x) / 2))
        while True:
            fx = my_function(guess)
            df = derivative(guess)
            if fx == None or df == None or df == 0:
                break
            guess -= fx / df
            print(guess)
            if guess < last_x or guess > x:
                break
            if my_function(guess) == None or abs(my_function(guess)) < Decimal("1e-100"):
                break
        print(abs(my_function(guess)))
        if not (guess < last_x or guess > x or abs(my_function(guess)) > Decimal("1e-50")):
            guess = guess.quantize(Decimal("10") ** -32, ROUND_HALF_UP)
            print(f"Root found at x = {guess}, f(x) = {my_function(guess)}")
            plt.scatter(guess, my_function(guess), color='red', s=10)
    if y == 0:
        plt.scatter(x, y, color='red', s=10)
    last_x = x
    last_y = y

plt.title(func_str)
plt.xlabel("x")
plt.ylabel("y")
plt.grid(True)
plt.xlim(-10, 10)
plt.ylim(-10, 10)
plt.axhline(0, color='black', linewidth=0.8)  # x-axis
plt.axvline(0, color='black', linewidth=0.8)  # y-axis
plt.savefig("graph.png")  # Save instead of show
print("Saved to graph.png")