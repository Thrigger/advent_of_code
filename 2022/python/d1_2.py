inputs = open("../inputs/d01.txt").read().strip().split("\n\n")

most = []
for line in inputs:
    val = sum(map(lambda x: int(x), line.split("\n")))
    most.append(val)

print(sum(sorted(most)[-3:len(most)]))
