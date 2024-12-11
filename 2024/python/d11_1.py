#inputs = open("../inputs/examples/d11.txt").read().strip().split("\n")
inputs = open("../inputs/d11.txt").read().strip().split("\n")

res = 0
stones = []
for line in inputs:
    stones = line.split(" ")

for i in range(25):
    j = 0
    while j < len(stones):
        if stones[j] == "0":
            stones[j] = "1"
        elif len(stones[j]) % 2 == 0:
            new_stone = stones[j][int(len(stones[j])/2):]
            new_stone = str(int(new_stone))
            stones[j] = stones[j][:int(len(stones[j])/2)]
            j += 1
            stones.insert(j, new_stone)
        else:
            stones[j] = str(int(stones[j]) * 2024)
        j += 1

print(len(stones))
