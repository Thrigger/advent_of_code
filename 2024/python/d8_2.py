#inputs = open("../inputs/examples/d08.txt").read().strip().split("\n")
inputs = open("../inputs/d08.txt").read().strip().split("\n")

res = 0
ant_map = []
for line in inputs:
    ant_map.append(list(line))

def is_inbound(amap, x, y):
    if y >= 0 and y < len(amap) and x >= 0 and x < len(amap[0]):
        return True
    return False

def mark_antinodes(amap, x, y):
    for j in range(y, len(amap)):
        for i in range(len(amap[j])):
            if j > y or i > x:
                if amap[j][i][:1] == amap[y][x][:1]:
                    dx = x-i
                    dy = y-j
                    new_x = x
                    new_y = y
                    while is_inbound(amap, new_x, new_y):
                        amap[new_y][new_x] += "#"
                        new_x = new_x + dx
                        new_y = new_y + dy
                    new_x = i
                    new_y = j
                    while is_inbound(amap, new_x, new_y):
                        amap[new_y][new_x] += "#"
                        new_x = new_x - dx
                        new_y = new_y - dy

for j in range(len(ant_map)):
    for i in range(len(ant_map[j])):
        node = ant_map[j][i][:1] 
        if node != ".":
            mark_antinodes(ant_map, i, j)


for line in ant_map:
    for each in line:
        if "#" in each:
            res += 1
print(res)
