#inputs = open("../inputs/examples/d02.txt").read().strip().split("\n")
inputs = open("../inputs/d02.txt").read().strip().split("\n")

res = 0
for line in inputs:
    they, me = line.split(" ")
    if me == "X":
        shape = 1
        if they == "A":
            score = 3
        elif they == "B":
            score = 0
        elif they == "C":
            score = 6
    if me == "Y":
        shape = 2
        if they == "A":
            score = 6
        elif they == "B":
            score = 3
        elif they == "C":
            score = 0
    if me == "Z":
        shape = 3
        if they == "A":
            score = 0
        elif they == "B":
            score = 6
        elif they == "C":
            score = 3
    res += shape+score

print(res)
