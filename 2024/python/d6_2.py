#inputs = open("../inputs/examples/d06.txt").read().strip().split("\n")
inputs = open("../inputs/d06.txt").read().strip().split("\n")

res = 0
room = []
start_y = 0
i = 0
start_x = 0
for line in inputs:
    current = list(line)
    room.append(current)
    if "^" in current:
        start_y = i
        start_x = current.index("^")
    i += 1

def check_room(r):
    x = start_x
    y = start_y
    while True:
        current = r[y][x][-1]
        if current == "^":
            if y-1 < 0:
                return False
            if "#" in r[y-1][x]:
                r[y][x] += ">"
            else:
                y -= 1
                if "^" in r[y][x]:
                    return True
                r[y][x] += "^"
        elif current == ">":
            if x + 1 == len(r[y]):
                return False
            if "#" in r[y][x+1]:
                r[y][x] += "v"
            else:
                x += 1
                if ">" in r[y][x]:
                    return True
                r[y][x] += ">"
        elif current == "v":
            if y + 1 == len(room):
                return False
            if "#" in r[y+1][x]:
                r[y][x] += "<"
            else:
                y += 1
                if "v" in r[y][x]:
                    return True
                r[y][x] += "v"
        elif current == "<":
            if x-1 < 0:
                return False
            if "#" in r[y][x-1]:
                r[y][x] += "^"
            else:
                x -= 1
                if "<" in r[y][x]:
                    return True
                r[y][x] += "<"

for yy in range(len(room)):
    print("Progress" + str(yy+1) + "/" +str(len(room)))
    for xx in range(len(room[yy])):
        if not (xx == start_x and yy == start_y) and "#" not in room[yy][xx]:
            new_room = []
            for line in room:
                new_room.append(line.copy())
            new_room[yy][xx] += "#";

            if check_room(new_room):
                res += 1

print(res)
