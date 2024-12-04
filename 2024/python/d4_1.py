#inputs = open("../inputs/examples/d04.txt").read().strip().split("\n")
inputs = open("../inputs/d04.txt").read().strip().split("\n")

res = 0
cross = []
for line in inputs:
    cross.append(list(line))

for j in range(len(cross)):
    line = cross[j]
    for i in range(len(line)):
        if i < len(line) - 3:
            if cross[j][i] == "X" and cross[j][i+1] == "M" and cross[j][i+2] == "A" and  cross[j][i+3] == "S":
                res += 1
            if cross[j][i+3] == "X" and cross[j][i+2] == "M" and cross[j][i+1] == "A" and  cross[j][i] == "S":
                res += 1
        if j < len(cross) - 3:
            if cross[j][i] == "X" and cross[j+1][i] == "M" and cross[j+2][i] == "A" and  cross[j+3][i] == "S":
                res += 1
            if cross[j+3][i] == "X" and cross[j+2][i] == "M" and cross[j+1][i] == "A" and  cross[j][i] == "S":
                res += 1
        if i < len(line) - 3 and j < len(cross) - 3:
            if cross[j][i] == "X" and cross[j+1][i+1] == "M" and cross[j+2][i+2] == "A" and  cross[j+3][i+3] == "S":
                res += 1
            if cross[j+3][i+3] == "X" and cross[j+2][i+2] == "M" and cross[j+1][i+1] == "A" and  cross[j][i] == "S":
                res += 1
            if cross[j+3][i] == "X" and cross[j+2][i+1] == "M" and cross[j+1][i+2] == "A" and  cross[j][i+3] == "S":
                res += 1
            if cross[j][i+3] == "X" and cross[j+1][i+2] == "M" and cross[j+2][i+1] == "A" and  cross[j+3][i] == "S":
                res += 1
print(res)
