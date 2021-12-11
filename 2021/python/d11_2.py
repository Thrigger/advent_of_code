inputs = open("../inputs/d11.txt").read().strip().split("\n")

data = []
for each in inputs:
    data.append(list(map(int, each)))

def noExplosions(data):
    for row in range(len(data)):
        for col in range(len(data)):
            if data[row][col] > 9:
                return True
    return False

def explode(data, row, col):
    data[row][col] = 0
    if row > 0 and col > 0:
        if data[row-1][col-1] != 0:
            data[row-1][col-1] += 1
    if row > 0:
        if data[row-1][col] != 0:
            data[row-1][col] += 1
    if row > 0 and col < len(data[0])-1:
        if data[row-1][col+1] != 0:
            data[row-1][col+1] += 1
    if col > 0:
        if data[row][col-1] != 0:
            data[row][col-1] += 1
    if col < len(data[0])-1:
        if data[row][col+1] != 0:
            data[row][col+1] += 1
    if row < len(data)-1 and col > 0:
        if data[row+1][col-1] != 0:
            data[row+1][col-1] += 1
    if row < len(data)-1:
        if data[row+1][col] != 0:
            data[row+1][col] += 1
    if row < len(data)-1 and col < len(data[0])-1:
        if data[row+1][col+1] != 0:
            data[row+1][col+1] += 1

step = 0
while step < 1000:
    allZero=True
    for row in range(len(data)):
        for col in range(len(data)):
            if data[row][col] != 0:
                allZero=False
    if allZero:
        break;

    for row in range(len(data)):
        for col in range(len(data)):
            data[row][col] += 1

    while (noExplosions(data)):
       for row in range(len(data)):
           for col in range(len(data)):
               if data[row][col] > 9:
                   explode(data, row, col)
    step += 1

print(step)

