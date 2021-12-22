inputs = open("../inputs/d15.txt").read().strip().split("\n")

data = []
offset = 0
count = 0
for _ in range(5):
    for each in inputs:
        line = list(map(int, each)) 
        size = len(line)
        new_line=[]
        for i in range(5):
            for digit in line:
                new_val = digit+i+offset
                if new_val > 18:
                    new_val-=9
                if new_val > 18:
                    new_val-=9
                if new_val > 18:
                    new_val -= 9
                if new_val > 9:
                    new_val = new_val%9
                new_line.append(new_val)

        data.append(new_line)
    offset += 1

mem = [[-1 for c in range(len(data[0]))] for i in range(len(data))]

def get_next_node(nodes):
    cost=-1
    point=-1
    for each in nodes:
        (x,y) = each
        if cost == -1 or mem[y][x] < cost:
            cost = mem[y][x]
            point=each
    return point

next_nodes=[(0,0)]
visited=[]


print("This function is to slow but works")
mem[0][0] = 0
for i in range(10000000):
    (x,y) = get_next_node(next_nodes)
    next_nodes.remove((x,y))
    data[y][x] = -1

    if y == len(data)-1 and x == len(data[0])-1:
        break

    if y < len(data)-1:
        if data[y+1][x] != -1:
            cost = mem[y][x] + data[y+1][x]  
            if mem[y+1][x] == -1 or mem[y+1][x] > cost:
                mem[y+1][x] = cost
            if (x,y+1) not in next_nodes:
                next_nodes.append((x,y+1))
    if x < len(data[0])-1:
        if data[y][x+1] != -1:
            cost = mem[y][x] + data[y][x+1]  
            if mem[y][x+1] == -1 or mem[y][x+1] > cost:
                mem[y][x+1] = cost
            if (x+1,y) not in next_nodes:
                next_nodes.append((x+1,y))
    if x > 0:
        if data[y][x-1] != -1:
            cost = mem[y][x] + data[y][x-1]  
            if mem[y][x-1] == -1 or mem[y][x-1] > cost:
                mem[y][x-1] = cost
            if (x-1,y) not in next_nodes:
                next_nodes.append((x-1,y))
    if y > 0:
        if data[y-1][x] != -1:
            cost = mem[y][x] + data[y-1][x]  
            if mem[y-1][x] == -1 or mem[y-1][x] > cost:
                mem[y-1][x] = cost
            if (x,y-1) not in next_nodes:
                next_nodes.append((x,y-1))

print(mem[len(data)-1][len(data[0])-1])


