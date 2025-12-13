#inputs = open("../inputs/examples/d12.txt").read().strip().split("\n")
inputs = open("../inputs/d12.txt").read().strip().split("\n")

res = 0
for line in inputs:
    region = [r for r in line.split(": ") if "x" in line]
    if region:
        cords = [int(c) for c in region[0].split("x")]
        pres = [int(p) for p in region[1].split(" ")]
        if cords[0]*cords[1]>=sum(pres)*9:
            res += 1

print(res)
