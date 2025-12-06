#inputs = open("../inputs/examples/d06.txt").read().strip().split("\n")
inputs = open("../inputs/d06.txt").read().strip().split("\n")

res = 0
problems = []
for line in inputs:
    if len(problems) == 0:
        for _l in line.split():
            problems.append([0])
    i = 0
    for val in line.split():
        if problems[i][0] == 0:
            problems[i][0] = int(val)
        elif val == "+":
            res += sum(problems[i])
        elif val == "*":
            tmp = 1
            for each in problems[i]:
                tmp *= each
            res += tmp
        else:
            problems[i].append(int(val))
        i += 1

print(res)
