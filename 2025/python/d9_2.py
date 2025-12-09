#inputs = open("../inputs/examples/d09.txt").read().strip().split("\n")
inputs = open("../inputs/d09.txt").read().strip().split("\n")

def is_ok(ls, x, y):
    for (l1,l2) in ls:
        # Same x in line
        if l1[0] == l2[0]:
            # x cross line
            if x[0] < l1[0] and x[1] > l1[0]:
                if len(range(max(y[0], l1[1]),min(y[1],l2[1])+1)) > 1:
                    return False
        # Same y in line
        elif l1[1] == l2[1]:
            # y cross line
            if y[0] < l1[1] and y[1] > l1[1]:
                if len(range(max(x[0], l1[0]),min(x[1],l2[0])+1)) > 1:
                    return False
    return True

tiles=[(line.split(",")) for line in inputs]
tiles=[[int(t[0]), int(t[1])] for t in tiles]

lines=[]
for i in range(len(tiles)):
    next_i =(i+1)%len(tiles)
    # line -
    if tiles[i][0] == tiles[next_i][0]:
        if tiles[i][1] > tiles[next_i][1]:
            lines.append((tiles[next_i], tiles[i]))
        else:
            lines.append((tiles[i], tiles[next_i]))
    # line |
    if tiles[i][1] == tiles[next_i][1]:
        if tiles[i][0] > tiles[next_i][0]:
            lines.append((tiles[next_i], tiles[i]))
        else:
            lines.append((tiles[i], tiles[next_i]))

res=0
# Compare all tiles
for i in range(len(tiles)):
    for j in range(i+1, len(tiles)):
        if tiles[i][0] < tiles[j][0]:
            xstart=tiles[i][0]
            xstop=tiles[j][0]
        elif tiles[i][0] >= tiles[j][0]:
            xstart=tiles[j][0]
            xstop=tiles[i][0]
        if tiles[i][1] < tiles[j][1]:
            ystart=tiles[i][1]
            ystop=tiles[j][1]
        elif tiles[i][1] >= tiles[j][1]:
            ystart=tiles[j][1]
            ystop=tiles[i][1]
        xs = (xstart, xstop)
        ys = (ystart, ystop)

        if is_ok(lines, xs, ys):
            size = (xstop-xstart+1)*(ystop-ystart+1)
            if size > res:
                res=size
print(res)
