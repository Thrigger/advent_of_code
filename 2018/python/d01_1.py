#inputs = open("../inputs/examples/d01.txt").read().strip().split("\n")
inputs = open("../inputs/d01.txt").read().strip().split("\n")

res = 0
for line in inputs:
    res += int(line)
print(res)
