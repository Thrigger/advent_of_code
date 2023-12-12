#inputs = open("../inputs/examples/d11.txt").read().strip().split("\n")
inputs = open("../inputs/d11.txt").read().strip().split("\n")

res = 0
image = []
for line in inputs:
    if line.count(".") == len(line):
        image.append(list(line))
    image.append(list(line))

col = 0
while col < len(image[0]):
    count = 0
    for row in range(len(image)):
        if image[row][col] == ".":
            count += 1
    if count == len(image):
        for row in range(len(image)):
            image[row].insert(col, ".")
        col += 2
    else:
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
        res += abs(each[0] - other[0])
        res += abs(each[1] - other[1])

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

