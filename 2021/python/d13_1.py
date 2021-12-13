inputs = open("../inputs/d13.txt").read().strip().split("\n\n")

dots_start=inputs[0].split("\n")
folds=inputs[1].split("\n")

rows = 2000
cols = 2000
dots = [["." for x in range(rows)] for y in range(cols)]
for dot in dots_start:
    (x,y) = dot.split(",")
    (x,y) = (int(x), int(y))
    dots[y][x]="#"

for fold in folds:

    (direction, line) = fold.split("=")
    direc= direction[-1:]
    line = int(line)

    if direc == "y":
        for row in range(line+1):
            for col in range(cols):
                if row == line:
                    dots[row][col] = "-"
                elif dots[2*line -row][col] == "#":
                    dots[2*line-row][col] = "."
                    dots[row][col] = "#"

    elif direc == "x":
        for row in range(rows):
            for col in range(line+1):
                if col == line:
                    dots[row][col] = "-"
                elif dots[row][2*line-col] == "#":
                    dots[row][2*line-col] = "."
                    dots[row][col] = "#"
res=0
for each in dots:
    res+=each.count("#")
print(res)

