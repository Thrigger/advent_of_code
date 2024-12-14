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
robots = []
for line in inputs:
    vals = line.split(" ")
    p = tuple( [ int(s) for s in vals[0].split("=")[1].split(",") ] )
    v = tuple( [ int(s) for s in vals[1].split("=")[1].split(",") ] )

    robots.append((p,v))

x_max = 101
y_max = 103
x_half = int(x_max/2)
y_half = int(y_max/2)
i = 6750
while True:
    room = [[0 for _ in range(x_max)] for _ in range(y_max)]
    for (p,v) in robots:
        end_p = (tup_add(p, tup_mul(v, i)))
        end_p = (end_p[0]%x_max, end_p[1]%y_max)
        room[end_p[1]][end_p[0]] += 1

    if sum(room[0]) < 5 and sum(room[1])<5 and sum(room[-1])<5 and sum(room[-2])<5:
        for line in room:
            l = ""
            for e in line:
                if e == 0:
                    l+=" "
                else:
                    l+= str(e)
            print(l)
        input(i)
    i += 1
