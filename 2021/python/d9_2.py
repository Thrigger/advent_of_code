inputs = open("../inputs/d09.txt").read().strip().split("\n")

def isLowPoint(tab, tmp, x, y):
    if tab[x][y] == 9:
        tmp[x][y] = 10
        return False
    if x > 0: 
        if tab[x][y] > tab[x-1][y] and tmp[x-1][y] != 10:
            return False
    if x < len(tab) - 1:
        if tab[x][y] > tab[x+1][y] and tmp[x+1][y] != 10:
            return False
    if y > 0: 
        if tab[x][y] > tab[x][y-1] and tmp[x][y-1] != 10:
            return False
    if y < len(tab[0]) - 1:
        if tab[x][y] > tab[x][y+1] and tmp[x][y+1] != 10:
            return False
    return True

data = [[0 for x in range(len(inputs[0]))] for y in range(len(inputs))]
for i in range(len(data)):
    for j, char in enumerate(inputs[i]):
        data[i][j]=int(char)

def getBasins(tab, tmp, x, y):
    bas = 0
    if isLowPoint(data, tmp, x, y):
        bas = 1
        tmp[x][y] = 10
    else:
        return 0

    if x > 0: 
        if tmp[x-1][y] != 10 and tab[x][y] <= tab[x-1][y]:
            bas += getBasins(data, tmp, x-1, y)
    if x < len(tab) - 1:
        if tmp[x+1][y] != 10 and tab[x][y] <= tab[x+1][y]:
            bas += getBasins(data, tmp, x+1, y)
    if y > 0: 
        if tmp[x][y-1] != 10 and tab[x][y] <= tab[x][y-1]:
            bas += getBasins(data, tmp, x, y-1)
    if y < len(tab[0]) - 1:
        if tmp[x][y+1] != 10 and tab[x][y] <= tab[x][y+1]:
            bas += getBasins(data, tmp, x, y+1)

    return bas

top3 = []
tmp = [[0 for x in range(len(inputs[0]))] for y in range(len(inputs))]
for i in range(len(data)):
    for j, char in enumerate(data[0]):
        if isLowPoint(data, tmp, i, j):
            bas = getBasins(data, tmp, i, j)
            top3.append(bas)

res = 1
for i in range(3):
    current = max(top3)
    res *= current
    top3.remove(current)
print(res)


