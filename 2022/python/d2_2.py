#inputs = open("../inputs/examples/d02.txt").read().strip().split("\n")
inputs = open("../inputs/d02.txt").read().strip().split("\n")

res = 0
for line in inputs:
    they, me = line.split(" ")
    if me == "X":
        score=0
        if they == "A":
            shape = 3
        elif they == "B":
            shape = 1
        elif they == "C":
            shape = 2
    if me == "Y":
        score=3
        if they == "A":
            shape = 1
        elif they == "B":
            shape = 2
        elif they == "C":
            shape = 3
    if me == "Z":
        score= 6
        if they == "A":
            shape = 2
        elif they == "B":
            shape = 3 
        elif they == "C":
            shape = 1
    res += shape+score
print(res)
