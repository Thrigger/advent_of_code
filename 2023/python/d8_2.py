#inputs = open("../inputs/examples/d08.txt").read().strip().split("\n\n")
#inputs = open("../inputs/examples/d08b.txt").read().strip().split("\n\n")
#inputs = open("../inputs/examples/d08c.txt").read().strip().split("\n\n")
inputs = open("../inputs/d08.txt").read().strip().split("\n\n")

def steps_to_goal(start):
    res = 0
    key = start
    goal = start#"ZZA"
    while not key[2] == "Z":
        vals = manual[key]
        if inst[res%len(inst)] == "L":
            key = vals[0]
        else:
            key = vals[1]
        res += 1
    return res

inst = list(inputs[0])

manual = {}
for line in inputs[1].split("\n"):
    key, vals = line.split(" = ")
    vals = vals[1:-1].split(", ")
    manual[key] = (vals)

starts = []
for each in manual:
    if each[2] == "A":
        starts.append(each)

res = []
for start in starts:
    res.append(steps_to_goal(start))

from math import lcm
print(lcm(*res))

