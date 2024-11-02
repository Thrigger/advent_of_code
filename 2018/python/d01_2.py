#inputs = open("../inputs/examples/d01.txt").read().strip().split("\n")
inputs = open("../inputs/d01.txt").read().strip().split("\n")

res = 0
seen = [False] * 10000000
i = 0
while True:
    res += int(inputs[i])
    if seen[res]:
        print(res)
        break
    seen[res] = True
    i = (i +1) % len(inputs)
