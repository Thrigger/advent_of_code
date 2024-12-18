#inputs = open("../inputs/examples/d01.txt").read().strip().split("\n")
inputs = open("../inputs/d01.txt").read().strip().split("\n")

res = 0
left = []
right = []
for line in inputs:
    l, r = line.split()
    left.append(int(l))
    right.append(int(r))
left.sort()
right.sort()

for (l, r) in zip(left, right):
    res += abs(l-r)

print(res)
