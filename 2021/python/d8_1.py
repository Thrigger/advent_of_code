inputs = open("../inputs/d08.txt").read().strip().split("\n")

data = []
for each in inputs:
    data.append(each.split(" | "))

res = 0 
for each in data:
    outs = each[1].split(" ")
    for out in outs:
        if len(out) == 2:
            res+=1
        elif len(out) == 4:
            res+=1
        elif len(out) == 3:
            res+=1
        elif len(out) == 7:
            res+=1

print(res)

