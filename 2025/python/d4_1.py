#inputs = open("../inputs/examples/d04.txt").read().strip().split("\n")
inputs = open("../inputs/d04.txt").read().strip().split("\n")

def movable(grid, x, y):
    if grid[y][x] == ".":
        return False
    res = 0
    if x > 0:
        if grid[y][x-1] == "@": 
            res += 1
        if y > 0:
            if grid[y-1][x-1] == "@": 
                res += 1
        if y < len(grid) -1:
            if grid[y+1][x-1] == "@": 
                res += 1
    if x < len(grid[0])-1:
        if grid[y][x+1] == "@": 
            res += 1
        if y > 0:
            if grid[y-1][x+1] == "@": 
                res += 1
        if y < len(grid) -1:
            if grid[y+1][x+1] == "@":
                res += 1
    if y > 0:
        if grid[y-1][x] == "@": 
            res += 1
    if y < len(grid) -1:
        if grid[y+1][x] == "@": 
            res += 1

    if res < 4:
        return True
    return False

res = 0
grid = []
for line in inputs:
    grid.append(list(line))

for x in range(len(grid[0])):
    for y in range(len(grid)):
        if movable(grid, x, y):
            res += 1

print(res)
