#inputs = open("../inputs/examples/d04.txt").read().strip().split("\n")
inputs = open("../inputs/d04.txt").read().strip().split("\n")

res = 0
for line in inputs:
    _, data = line.split(": ")
    wins, my = data.split(" | ")
    wins = wins.split()
    my = my.split()
    overlaps = len(set(wins) & set(my))
    
    points = 1
    if overlaps == 0:
        points = 0

    for i in range(overlaps - 1):
        points *= 2

    res += points

print(res)

