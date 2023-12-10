#inputs = open("../inputs/examples/d10.txt").read().strip().split("\n")
#inputs = open("../inputs/examples/d10b.txt").read().strip().split("\n")
#inputs = open("../inputs/examples/d10c.txt").read().strip().split("\n")
#inputs = open("../inputs/examples/d10d.txt").read().strip().split("\n")
inputs = open("../inputs/d10.txt").read().strip().split("\n")

def get_next(field, path):
    row, col = path[-1]
    t = field[row][col]
    
    p1, p2 = get_pot_points(t, row, col)
    if p1 in path:
        return p2
    return p1
    
def get_pot_points(t, r, c):
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

# Find start square
for row in range(len(field)):
    for col in range(len(field[0])):
        if field[row][col] == "S":
            start_row = row
            start_col = col
            break

paths=[]
# Get the two paths by bruteforcing
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

# Loop until paths are connected
while paths[0][-1] != paths[1][-1]:
    for path in paths:
        path.append(get_next(field, path))

paths[0].extend(reversed(paths[1][1:-1]))
loop = paths[0]

# Part 2 function
def count_crossings(row, col):
    res = 0
    right = 0
    left = 0

    for i in reversed(range(row)):
        if [i, col] in loop:
            if field[i][col] == "-":
                res += 1
            elif field[i][col] in ["F", "L"]:
                right += 1
            elif field[i][col] in ["J", "7"]:
                left += 1
        elif field[i][col] == "O":
            break
        elif field[i][col] == "I":
            res += 1
            break

    # This is a bit of magic.
    # But looking at the map the right and left turn "compliment each other" creating an "-"
    # And 2 right (or left) will create a loop that can be "passed by"
    res += min(left, right)

    return res

inside = 0
for row in range(len(field)):
    for col in range(len(field[0])):
        if not [row, col] in loop:
            if count_crossings(row, col) % 2 != 0:
                inside += 1
                field[row][col] = "I"
            else:
                field[row][col] = "O"

print(inside)

