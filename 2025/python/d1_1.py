#inputs = open("../inputs/examples/d01.txt").read().strip().split("\n")
inputs = open("../inputs/d01.txt").read().strip().split("\n")

res = 0
vault = 50
for line in inputs:
    c = line[0]
    val = int(line[1:])
    if c == "L":
        vault -= val
    elif c == "R":
        vault += val
    vault %= 100    
    
    if vault == 0:
        res += 1
print(res)
