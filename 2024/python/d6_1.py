#inputs = open("../inputs/examples/d06.txt").read().strip().split("\n")
inputs = open("../inputs/d06.txt").read().strip().split("\n")

res = 0
room = []
y = 0
i = 0
x = 0
for line in inputs:
    current = list(line)
    room.append(current)
    if "^" in current:
        y = i
        x = current.index("^")
    i += 1
        
while True:
    if room[y][x] == "^":
        if y-1 < 0:
            room[y][x] = "X"
            break
        if room[y-1][x] == "#":
            room[y][x] = ">"
        else:
            room[y][x] = "X"
            y -= 1
            room[y][x] = "^"
    elif room[y][x] == ">":
        if x + 1 == len(room[y]):
            room[y][x] = "X"
            break
        if room[y][x+1] == "#":
            room[y][x] = "v"
        else:
            room[y][x] = "X"
            x += 1
            room[y][x] = ">"
    elif room[y][x] == "v":
        if y + 1 == len(room):
            room[y][x] = "X"
            break
        if room[y+1][x] == "#":
            room[y][x] = "<"
        else:
            room[y][x] = "X"
            y += 1
            room[y][x] = "v"
    elif room[y][x] == "<":
        if x-1 < 0:
            room[y][x] = "X"
            break
        if room[y][x-1] == "#":
            room[y][x] = "^"
        else:
            room[y][x] = "X"
            x -= 1
            room[y][x] = "<"

for line in room:
    res += line.count("X")
    
print(res)
