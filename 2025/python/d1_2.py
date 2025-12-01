#inputs = open("../inputs/examples/d01.txt").read().strip().split("\n")
inputs = open("../inputs/d01.txt").read().strip().split("\n")

res = 0
vault = 50
for line in inputs:
    c = line[0]
    val = int(line[1:])
    old = vault

    res += val//100
    val %= 100

    if c == "L":
        vault -= val
    elif c == "R":
        vault += val
    
    if old == 0:
        vault %= 100    
    elif vault <= 0 or vault > 99:
        res += 1
        vault %= 100    

print(res)
