#inputs = open("../inputs/examples/d14.txt").read().strip().split("\n")
inputs = open("../inputs/d14.txt").read().strip().split("\n")

def tup_sub(a,b):
    return (a[0]-b[0], a[1]-b[1])
def tup_add(a,b):
    return (a[0]+b[0], a[1]+b[1])
def tup_mul(t,a):
    return (a*t[0], a*t[1])
def tup_pos(t):
    return t[0] >= 0 and t[1] >= 0
   
res = [0, 0, 0, 0]
for line in inputs:
    vals = line.split(" ")
    p = tuple( [ int(s) for s in vals[0].split("=")[1].split(",") ] )
    v = tuple( [ int(s) for s in vals[1].split("=")[1].split(",") ] )

    x_max = 101
    y_max = 103
    x_half = int(x_max/2)
    y_half = int(y_max/2)

    end_p = (tup_add(p, tup_mul(v, 100)))
    end_p = (end_p[0]%x_max, end_p[1]%y_max)

    if end_p[0] < x_half and end_p[1] < y_half:
        res[0] += 1
    elif end_p[0] < x_half and end_p[1] > y_half:
        res[1] += 1
    elif end_p[0] > x_half and end_p[1] < y_half:
        res[2] += 1
    elif end_p[0] > x_half and end_p[1] > y_half:
        res[3] += 1

total = 1
for each in res:
    total *= each

print(total)
