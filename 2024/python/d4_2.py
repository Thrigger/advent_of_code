#inputs = open("../inputs/examples/d04.txt").read().strip().split("\n")
inputs = open("../inputs/d04.txt").read().strip().split("\n")

res = 0
cross = []
for line in inputs:
    cross.append(list(line))

for j in range(len(cross)):
    line = cross[j]
    for i in range(len(line)):
        if i < len(line) - 2 and j < len(cross) - 2:
            if cross[j][i] == "M" and cross[j][i+2] == "M" and cross[j+1][i+1] == "A" and  cross[j+2][i] == "S" and  cross[j+2][i+2] == "S":
                res += 1
            if cross[j][i] == "S" and cross[j][i+2] == "M" and cross[j+1][i+1] == "A" and  cross[j+2][i] == "S" and  cross[j+2][i+2] == "M":
                res += 1
            if cross[j][i] == "S" and cross[j][i+2] == "S" and cross[j+1][i+1] == "A" and  cross[j+2][i] == "M" and  cross[j+2][i+2] == "M":
                res += 1
            if cross[j][i] == "M" and cross[j][i+2] == "S" and cross[j+1][i+1] == "A" and  cross[j+2][i] == "M" and  cross[j+2][i+2] == "S":
                res += 1
print(res)
