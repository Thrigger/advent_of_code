#inputs = open("../inputs/examples/d01.txt").read().strip().split("\n")
inputs = open("../inputs/d01.txt").read().strip().split("\n")

res = 0
for line in inputs:
    last = 0;
    first = 10;
    for each in line:
        if each.isnumeric():
            last = int(each)
            if first == 10:
                first = int(each)
    val = int(str(first)+str(last))
    res += val

print(res)

