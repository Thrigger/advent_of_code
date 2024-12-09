#inputs = open("../inputs/examples/d09.txt").read().strip().split("\n")
inputs = open("../inputs/d09.txt").read().strip().split("\n")

res = 0
i = 0
disk = []
for each in inputs[0]:
    each = int(each)
    if i % 2 == 0:
        for j in range(each):
            disk.append(str(int(i/2)))
    else:
        for j in range(each):
            disk.append(".")
    i += 1

i = 0
while i < len(disk):
    if disk[i] == ".":
        if i % 100 == 0:
            print("prog: " + str(i+1) +"/"+str(len(disk)))
        disk[i] = disk[-1]
        disk[-1] = "."
        while disk[-1] == ".":
            disk = disk[:-1]
    res += i * int(disk[i])
    i += 1

print("This is not an elegant solution, part 2 is better")
print(res)
