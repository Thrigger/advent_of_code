
inputs = open("../inputs/d02.txt").read().strip().split("\n")

hor = 0
dep = 0
for inst in inputs:
    direc, steps = inst.split(" ")

    if direc == "forward":
        hor += int(steps)
    elif direc == "up":
        dep -= int(steps)
    elif direc == "down":
        dep += int(steps)
print(dep*hor)
