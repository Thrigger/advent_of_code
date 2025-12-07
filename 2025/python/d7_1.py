#inputs = open("../inputs/examples/d07.txt").read().strip().split("\n")
inputs = open("../inputs/d07.txt").read().strip().split("\n")

res = 0
grid = [list(row) for row in inputs]

y = 1
while y < len(grid):
    for x in range(len(grid[0])):
        if grid[y-1][x] in ["S", "|"]:
            if grid[y][x] == "^":
                res += 1
                grid[y][x-1] = "|"
                grid[y][x+1] = "|"
            else:
                grid[y][x] = "|"
    y+=1

print(res)
