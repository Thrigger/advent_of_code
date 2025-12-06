#inputs = open("../inputs/examples/d06.txt").read().split("\n")
inputs = open("../inputs/d06.txt").read().split("\n")

grid = []
for line in inputs:
    if line != "":
        grid.append(line)

new_in = ""
x = len(grid[0])-1
while x >= 0:
    val = ""
    for y in range(len(grid)):
        val += grid[y][x]
    print(val)
    new_in += val
    if val.find("*") >= 0 or val.find("+") >= 0:
        new_in += "\n"
        x-=1
    x-=1
    
res = 0
for line in new_in.split("\n"):
    start = 0
    vals = []
    while start < len(line):
        first = line[start]
        if first == "+":
            res += sum(vals)
            break
        elif first == "*":
            tmp = 1
            for each in vals:
                tmp *= each
            res += tmp
            break
        if line[start:start+4] == "   ":
            vals.append(0)
        else:
            vals.append(int(line[start:start+4]))
        start += 4
        if line[start]==" ":
            start +=1

print(res)
