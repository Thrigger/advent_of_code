#inputs = open("../inputs/examples/d15b.txt").read().strip().split("\n\n")
#inputs = open("../inputs/examples/d15.txt").read().strip().split("\n\n")
inputs = open("../inputs/d15.txt").read().strip().split("\n\n")

res = 0
room = []
y = 0
for line in inputs[0].split("\n"):
    room.append(list(line))
    if "@" in room[y]:
        r_x = room[y].index("@")
        r_y = y
    y+= 1
print(room)

mvs = list(inputs[1].replace("\n", ""))

def no_walls(r_x, r_y, each):
    if each == "<":
        match room[r_y][r_x-1]:
            case "#":
                return False
            case ".":
                return True
            case "O":
                return no_walls(r_x-1, r_y, each)
    elif each == ">":
        match room[r_y][r_x+1]:
            case "#":
                return False
            case ".":
                return True
            case "O":
                return no_walls(r_x+1, r_y, each)
    elif each == "^":
        match room[r_y-1][r_x]:
            case "#":
                return False
            case ".":
                return True
            case "O":
                return no_walls(r_x, r_y-1, each)
    elif each == "v":
        match room[r_y+1][r_x]:
            case "#":
                return False
            case ".":
                return True
            case "O":
                return no_walls(r_x, r_y+1, each)

def move_all(r_x, r_y, each):
    if room[r_y][r_x] in ["#", "."]:
        return (r_x,r_y)
    if each == "<":
        move_all(r_x-1,r_y,each)
        room[r_y][r_x-1] = room[r_y][r_x]
        room[r_y][r_x] = "."
        return (r_x-1,r_y)
    elif each == ">":
        move_all(r_x+1,r_y,each)
        room[r_y][r_x+1] = room[r_y][r_x]
        room[r_y][r_x] = "."
        return (r_x+1,r_y)
    elif each == "^":
        move_all(r_x,r_y-1,each)
        room[r_y-1][r_x] = room[r_y][r_x]
        room[r_y][r_x] = "."
        return (r_x,r_y-1)
    elif each == "v":
        move_all(r_x,r_y+1,each)
        room[r_y+1][r_x] = room[r_y][r_x]
        room[r_y][r_x] = "."
        return (r_x,r_y+1)

for each in mvs:
    if no_walls(r_x, r_y, each):
        (r_x, r_y) = move_all(r_x, r_y, each)
#    for l in room:
#        print("".join(l))
    
for j in range(len(room)):
    for i in range(len(room[0])):
        if room[j][i] == "O":
            res += 100*j+i


print(res)

#              function     itterator
#map(lambda x: int(x), line.split("\n"))
#
#       lists
#vec = line.split(" ")
#vec = list(b)
#vec.append(a)
#len(vec)
#sorted(vec)   <- small to big
#print(vec)
#print(vec[0])
#list(set(some_list) & set(some_other_list))    <-- check overlapping
#
#       Loop several lists
#for (l, r) in zip(left, right):

