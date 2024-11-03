#inputs = open("../inputs/examples/d02.txt").read().strip().split("\n")
inputs = open("../inputs/d02.txt").read().strip().split("\n")

sum2 = 0
sum3 = 0

for line in inputs:
    found2 = False
    found3 = False
    while len(line) > 0:
        first_char = line[0]
        count = line.count(first_char)
        line = line.replace(first_char, "")
        if count == 2:
            found2 = True
        elif count == 3:
            found3 = True
    if found2:
        sum2 += 1
    if found3:
        sum3 += 1

print(sum2*sum3)
