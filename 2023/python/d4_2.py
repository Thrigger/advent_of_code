#inputs = open("../inputs/examples/d04.txt").read().strip().split("\n")
inputs = open("../inputs/d04.txt").read().strip().split("\n")

res = 0
multi= [1] * len(inputs)
i = 0
for line in inputs:
    _, data = line.split(": ")
    wins, my = data.split(" | ")
    wins = wins.split()
    my = my.split()
    new = len(set(wins) & set(my))

    for j in range(new):
        new_index = i + j + 1
        if new_index >= len(multi):
            break
        multi[new_index] += multi[i]

    i += 1

print(sum(multi))

