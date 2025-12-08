#inputs = open("../inputs/examples/d08.txt").read().strip().split("\n")
inputs = open("../inputs/d08.txt").read().strip().split("\n")

def length_between(p1, p2):
    total = (p1[0]-p2[0])**2 + (p1[1]-p2[1])**2 + (p1[2]-p2[2])**2
    return total**0.5

points=[]
for line in inputs:
    points.append([int(i) for i in line.split(",")])

connections=[]
for i in range(len(points)):
    for j in range(i, len(points)):
        if i != j:
            l=length_between(points[i], points[j])
            connections.append((l, points[i], points[j]))

connections = sorted(connections)
circs=[[p] for p in points]

while len(circs) > 1:
    current = connections.pop(0)
    i = 0
    while i < len(circs):
        if current[1] in circs[i] and current[2] in circs[i]:
            break
        elif current[1] in circs[i]:
            circs[i].append(current[2])
            for j in range(i+1, len(circs)):
                if current[2] in circs[j]:
                    over = circs.pop(j)
                    over.remove(current[2])
                    circs[i].extend(over)
                    break
            break
        elif current[2] in circs[i]:
            circs[i].append(current[1])
            for j in range(i+1, len(circs)):
                if current[1] in circs[j]:
                    over = circs.pop(j)
                    over.remove(current[1])
                    circs[i].extend(over)
                    break
            break
        else:
            i+=1

print(current[1][0]*current[2][0])
