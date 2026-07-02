import symbolicalgebra

while True:
    x = input(">>> ")
    if x == "exit":
        break
    z = symbolicalgebra.tree(symbolicalgebra.Parser(x).firstPass())
    print(z.evaluate())