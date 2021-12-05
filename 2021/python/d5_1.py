
# \n
#inputs = open("../inputs/examples/d05.txt").read().strip().split("\n")
inputs = open("../inputs/d05.txt").read().strip().split("\n")
# ,
#inputs = open("../inputs/examples/d05.txt").read().strip().split(" -> ")
#inputs = open("../inputs/d05.txt").read().strip().split("\n")

size = 1000
vent = [[0 for i in range(size)] for j in range(size)]
#print(vent)
for line in inputs:
    (start, stop) = line.split(" -> ")

    start_x, start_y = start.split(",")
    stop_x, stop_y = stop.split(",")

    start_x = int(start_x)
    start_y = int(start_y)
    stop_x = int(stop_x)
    stop_y = int(stop_y)
    if start_x == stop_x:
        if start_y < stop_y:
            for delta_y in range(start_y, stop_y+1):
                vent[start_x][delta_y] += 1
        else:
            for delta_y in range(stop_y, start_y+1):
                vent[start_x][delta_y] += 1
    elif start_y == stop_y:
        if start_x < stop_x:
            for delta_x in range(start_x, stop_x+1):
                vent[delta_x][start_y] += 1
        else:
            for delta_x in range(stop_x, start_x+1):
                vent[delta_x][start_y] += 1

    
res=0
res2=0
for row in vent:
    res += row.count(2)
    res += row.count(3)
    res += row.count(4)
    res += row.count(5)
    res += row.count(6)
    res2 += row.count(0)
    res2 += row.count(1)
print(1000000 - res2)
print(res)












