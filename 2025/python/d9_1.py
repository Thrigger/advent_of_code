#inputs = open("../inputs/examples/d09.txt").read().strip().split("\n")
inputs = open("../inputs/d09.txt").read().strip().split("\n")

res = 0
tiles=[(line.split(",")) for line in inputs]
tiles=[[int(t[0]), int(t[1])] for t in tiles]

res = 0
for i in range(len(tiles)):
    for j in range(i+1, len(tiles)):
        xdelta = abs(tiles[i][0]-tiles[j][0])+1
        ydelta = abs(tiles[i][1]-tiles[j][1])+1
        size = xdelta*ydelta
        if size > res:
            res=size
print(res)
