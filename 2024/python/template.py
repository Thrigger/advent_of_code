inputs = open("../inputs/examples/d01.txt").read().strip().split("\n")
#inputs = open("../inputs/d01.txt").read().strip().split("\n")

res = 0
for line in inputs:
    print(line)

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

