#inputs = open("../inputs/examples/d12.txt").read().strip().split("\n")
inputs = open("../inputs/d12.txt").read().strip().split("\n")

res = 0
garden_map = []
for line in inputs:
    garden_map.append(list(line))

def find_region(gmap, x, y):
    val = gmap[y][x]

    if "0" in val:
        return (0, 0)
    gmap[y][x] += "0"
    area = 1
    perimeter = 0

    if x > 0 and val in gmap[y][x-1]:
            a, p = find_region(gmap, x-1, y)
            area += a
            perimeter += p
    else:
        perimeter += 1
    if x+1 < len(gmap[0]) and val in gmap[y][x+1]:
            a, p = find_region(gmap, x+1, y)
            area += a
            perimeter += p
    else:
        perimeter += 1
    if y > 0 and val in gmap[y-1][x]:
            a, p = find_region(gmap, x, y-1)
            area += a
            perimeter += p
    else:
        perimeter += 1
    if y+1 < len(gmap) and val in gmap[y+1][x]:
            a, p = find_region(gmap, x, y+1)
            area += a
            perimeter += p
    else:
        perimeter += 1
    return (area, perimeter)

for j in range(len(garden_map)):
    for i in range(len(garden_map[j])):
        if "0" not in garden_map[j][i]:
            area, peri = find_region(garden_map, i, j)
            res += area*peri

print(res)
