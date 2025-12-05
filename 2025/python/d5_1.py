#inputs = open("../inputs/examples/d05.txt").read().strip().split("\n\n")
inputs = open("../inputs/d05.txt").read().strip().split("\n\n")

res = 0
fresh = []
for line in inputs[0].split("\n"):
    start, stop = line.split("-")
    fresh.append([int(start), int(stop)])

res = 0
for line in inputs[1].split("\n"):
    val = int(line)
    for each in fresh:
        if val >= each[0] and val <= each[1]:
            res += 1
            break

print(res)
