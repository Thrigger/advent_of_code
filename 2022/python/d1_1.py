inputs = open("../inputs/d01.txt").read().strip().split("\n\n")

most = 0
for line in inputs:
    val = sum(map(lambda x: int(x), line.split("\n")))
    if val > most:
        most = val

print(most)
