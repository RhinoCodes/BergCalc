x = [3,1,4,1] + [0] * 29
y = 5

i = 0
while i < len(x) - 1:
        if x[i] < y:
            print(x)
            x[i+1] += x[i] * 10
            x[i] = 0
            print(x)
        else:
            print(x)
            z = 0
            p = 0
            q = 0
            while z < x[i]:
                z += y
                p += 1
            if z > x[i]:
                z -= y
                p -= 1
            x[i+1] += (x[i] - z) * 10
            x[i] = p
        i+= 1

while i >= 0:
    while x[i] >= 10:
        x[i] -= 10
        x[i-1] += 1
    i-=1