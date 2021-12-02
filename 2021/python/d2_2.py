
inputs = open("../inputs/d02.txt").read().strip().split("\n")

hor = 0
dep = 0
aim = 0
for inst in inputs:
    direc, steps = inst.split(" ")

    if direc == "forward":
        hor += int(steps)
        dep += aim * int(steps)
    elif direc == "up":
        aim -= int(steps)
    elif direc == "down":
        aim += int(steps)
print(dep*hor)
