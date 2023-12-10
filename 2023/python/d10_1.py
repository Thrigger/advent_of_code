#inputs = open("../inputs/examples/d10.txt").read().strip().split("\n")
#inputs = open("../inputs/examples/d10b.txt").read().strip().split("\n")
inputs = open("../inputs/d10.txt").read().strip().split("\n")


def get_next(field, path):
    row, col = path[-1]
    t = field[row][col]
    
    p1, p2 = get_pot_points(field, t, row, col)
    if p1 in path:
        return p2
    return p1
    
def get_pot_points(f, t, r, c):
    if t == "F":
        return ( [r, c+1], [r+1, c] )
    elif t == "7":
        return ( [r, c-1], [r+1, c] )
    elif t == "L":
        return ( [r, c+1], [r-1, c] )
    elif t == "J":
        return ( [r, c-1], [r-1, c] )
    elif t == "|":
        return ( [r+1, c], [r-1, c] )
    elif t == "-":
        return ( [r, c+1], [r, c-1] )


field = [ list(line) for line in inputs ]

for row in range(len(field)):
    for col in range(len(field[0])):
        if field[row][col] == "S":
            start_row = row
            start_col = col
            break

paths=[]
# scan for incoming pipes to start
if start_row > 0 and (field[start_row - 1][start_col] in ["F", "7", "|"]):
    paths.append( [ [start_row, start_col] ] )
    paths[-1].append([start_row-1, start_col])
if start_row < len(field) - 1 and (field[start_row + 1][start_col] in ["L", "J", "|"]):
    paths.append( [ [start_row, start_col] ] )
    paths[-1].append([start_row+1, start_col])
if start_col > 0 and (field[start_row][start_col - 1] in ["F", "L", "-"]):
    paths.append( [ [start_row, start_col] ] )
    paths[-1].append([start_row, start_col-1])
if start_col < len(field[0]) -1 and (field[start_row][start_col + 1] in ["7", "J", "-"]):
    paths.append( [ [start_row, start_col] ] )
    paths[-1].append([start_row, start_col+1])

# lopp till on same square again
while paths[0][-1] != paths[1][-1]:
    for path in paths:
        print(len(path))
        path.append(get_next(field, path))

print(len(paths[0]) - 1)

