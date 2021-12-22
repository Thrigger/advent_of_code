inputs = open("../inputs/examples/d15.txt").read().strip().split("\n")

data = []
for each in inputs:
    data.append(list(map(int, each)))

mem = [[-1 for c in range(len(data[0]))] for i in range(len(data))]

finished=[]

def cost_of_path_full(x, y, cost):
    if mem[y][x] != -1 and cost > mem[y][x]:
        return
    else:
        mem[y][x] = cost

    if x == len(data[0])-1 and y == len(data)-1:
        finished.append(cost)
        return
    else:
        if x < len(data[0])-1:
            cost_of_path_full(x+1,y,cost+data[y][x+1])
        if y < len(data)-1:
            cost_of_path_full(x,y+1,cost+data[y+1][x])
        if x > 0:
            cost_of_path_full(x-1,y,cost+data[y][x-1])
        if y > 0:
            cost_of_path_full(x,y-1,cost+data[y-1][x])

cost_of_path_full(0,0,0)
print(min(finished))


