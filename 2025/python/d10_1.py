#inputs = open("../inputs/examples/d10.txt").read().strip().split("\n")
inputs = open("../inputs/d10.txt").read().strip().split("\n")

max_rec=6
best=6

def new_state(c, b):
    new = [bo for bo in c]
    for each in b:
        new[each] = not c[each]
    return new

def solve(ls, bs, c, ps):
    if ls == c:
        return ps
    if ps >= max_rec:
        return 0

    res = 10000
    for b in bs:
        new = new_state(c, b)
        tmp = solve(ls, bs, new, ps+1)
        if tmp > 0 and tmp < res:
            res = tmp

    if res == 10000:
        return 0
    return res

res = 0
for line in inputs:
    machine = line.split("]")
    lights = []
    current = []
    for l in machine[0]:
        if l == ".":
            lights.append(False)
            current.append(False)
        elif l == "#":
            lights.append(True)
            current.append(False)
    machine = machine[1].split("{")
    buttons = []
    for group in machine[0][1:-1].split(" "):
        buttons.append([int(val) for val in group[1:-1].split(",")])
        
    joltage = machine[1][-1].split(",")
    presses = 0
    tmp = solve(lights, buttons, current, presses)
    res += tmp

print(res)
