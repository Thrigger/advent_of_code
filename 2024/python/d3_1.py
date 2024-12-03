#inputs = open("../inputs/examples/d03.txt").read().strip().split("\n")
inputs = open("../inputs/d03.txt").read().strip().split("\n")

import re

res = 0
for line in inputs:
    x = re.findall(r'mul\(([0-9]{1,3}),([0-9]{1,3})\)', line);
    for each in x:
        res += int(each[0]) * int(each[1])

print(res)
