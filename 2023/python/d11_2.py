#inputs = open("../inputs/examples/d11.txt").read().strip().split("\n")
inputs = open("../inputs/d11.txt").read().strip().split("\n")


res = 0
image = []
for line in inputs:
    if line.count(".") == len(line):
        image.append(["X"] * len(line))
    else:
        image.append(list(line))

col = 0
while col < len(image[0]):
    count = 0
    for row in range(len(image)):
        if image[row][col] in [".", "X"]:
            count += 1
    if count == len(image):
        for row in range(len(image)):
            image[row][col] = "X"
    col += 1

for line in image:
    print(line)


galaxys = []
for row in range(len(image)):
    for col in range(len(image[0])):
        if image[row][col] == "#":
            galaxys.append( (row, col) )

print(galaxys)
print(len(galaxys))

while len(galaxys) > 0:
    each = galaxys[0]
    print(each)
    galaxys.remove(each)
    for other in galaxys:
        if each[0] != other[0]:
            for step_r in range(each[0], other[0], int((other[0] - each[0])/abs(other[0] -each[0]))):
                if image[step_r][each[1]] == "X":
                    res += 1000000
                else:
                    res += 1
        if each[1] != other[1]:
            for step_c in range(each[1], other[1], int((other[1] - each[1])/abs(other[1] -each[1]))):
                if image[each[0]][step_c] == "X":
                    res += 1000000
                else:
                    res += 1


print(res)

#              function     itterator
#map(lambda x: int(x), line.split("\n"))
#vec = line.split(" ")
#vec = list(b)
#vec.append(a)
#len(vec)
#sorted(vec)   <- small to big
#print(vec)
#print(vec[0])
#list(set(some_list) & set(some_other_list))    <-- check overlapping

