#inputs = open("../inputs/examples/d07.txt").read().strip().split("\n")
inputs = open("../inputs/d07.txt").read().strip().split("\n")

def nof_splits(g, mem, start_y):
    splits = 0
    y = start_y
    while y < len(g):
        for x in range(len(g[0])):
            if g[y-1][x] in ["S", "|"]:
                g[y-1][x] = "."
                if g[y][x] == "^":
                    if (x,y) in mem.keys():
                        splits += mem[(x,y)]
                    else:
                        g[y][x-1] = "|"
                        splits += nof_splits(g, mem, y+1)
                        g[y][x+1] = "|"
                        splits += nof_splits(g, mem, y+1)

                        mem[(x,y)] = splits
                    return splits
                else:
                    g[y][x] = "|"
        y+=1
    return 1
        
mem = {}
grid = [list(row) for row in inputs]

print(nof_splits(grid,mem,1))
