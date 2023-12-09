#inputs = open("../inputs/examples/d08.txt").read().strip().split("\n\n")
#inputs = open("../inputs/examples/d08b.txt").read().strip().split("\n\n")
inputs = open("../inputs/d08.txt").read().strip().split("\n\n")

inst = list(inputs[0])

res = 0
manual = {}
for line in inputs[1].split("\n"):
    key, vals = line.split(" = ")
    vals = vals[1:-1].split(", ")
    manual[key] = (vals)

key = "AAA"
while key != "ZZZ":
    vals = manual[key]
    if inst[res%len(inst)] == "L":
        key = vals[0]
    else:
        key = vals[1]
    res += 1

print(res)

