#inputs = open("../inputs/examples/d10.txt").read().strip().split("\n")
inputs = open("../inputs/d10.txt").read().strip().split("\n")

res = 0
trail_map = []
for line in inputs:
    trail_map.append([int(item) for item in line])

def find_paths(tmap, x, y):
    val = tmap[y][x]
    ret_val = []
    if val == 9:
        return [(x, y)]
    if x > 0 and tmap[y][x-1] == val + 1:
        ret_val += find_paths(tmap, x-1, y)
    if x+1 < len(tmap[0]) and tmap[y][x+1] == val + 1:
        ret_val += find_paths(tmap, x+1, y)
    if y > 0 and tmap[y-1][x] == val + 1:
        ret_val += find_paths(tmap, x, y-1)
    if y+1 < len(tmap) and tmap[y+1][x] == val + 1:
        ret_val += find_paths(tmap, x, y+1)
    return ret_val

for j in range(len(trail_map)):
    for i in range(len(trail_map[j])):
        node = trail_map[j][i]
        if node == 0:
            vals = list(dict.fromkeys(find_paths(trail_map, i, j)))
            res += len(vals)

print(res)
