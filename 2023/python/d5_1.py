#inputs = open("../inputs/examples/d05.txt").read().strip().split("\n\n")
inputs = open("../inputs/d05.txt").read().strip().split("\n\n")

for i in range(len(inputs)):
    group = inputs[i]
    if i == 0:
        _, vals = group.split(": ")
        vals = list(map(lambda x: int(x), vals.split()))
        print("Seeds")
        print(vals)
    else:
        lines = group.split("\n")
        new_vals = []
        updated = [False] * len(vals)
        for line in lines[1:]:
            dst, src, steps = list(map(lambda x: int(x), line.split()))
            print(line)
            for j in range(len(vals)):
                if vals[j] >= src and vals[j] < src + steps and not updated[j]:
                    vals[j] = vals[j] - src + dst 
                    updated[j] = True
                    print(vals)
        print("next")
        print(vals)

print(min(vals))

#              function     itterator
#map(lambda x: int(x), line.split("\n"))
#vec = line.split(" ")
#vec = list(b)
#vec.append(a)
#len(vec)
#sorted(vec)   <- small to big
#print(vec)
#print(vec[0])

