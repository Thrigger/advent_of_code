inputs = open("../inputs/examples/d09.txt").read().strip().split("\n")
#inputs = open("../inputs/d09.txt").read().strip().split("\n")

res = 0
for line in inputs:
    vals = []
    vals.append([int(x) for x in line.split()])

    current = vals[0]
    while current.count(0) != len(current):
        new_vals = []
        for i in range(len(current) -1):
            new_vals.append(current[i+1] - current[i])
        vals.append(new_vals)
        current = vals[-1]

    current.insert(0, 0)
    for i in range(len(vals) - 2, -1, -1):
        vals[i].insert(0, vals[i][0] - vals[i+1][0])
        if i == 0:
            res += vals[i][0]

print(res)

