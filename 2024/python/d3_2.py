#inputs = open("../inputs/examples/d03b.txt").read().strip().split("\n")
inputs = open("../inputs/d03.txt").read().strip().split("\n")

import re

res = 0
running = True
for line in inputs:
    x = re.findall(r'mul\(([0-9]{1,3}),([0-9]{1,3})\)|(don\'t)\(\)|(do)\(\)', line);
    for each in x:
        if each[2] == "don't":
            running = False
        elif each[3] == "do":
            running = True
        elif running:
            res += int(each[0]) * int(each[1])

print(res)
