inputs = open("../inputs/d09.txt").read().strip().split("\n")

def isLowPoint(tab, x, y):
    if x > 0: 
        if tab[x][y] > tab[x-1][y]:
            return False
    if x < len(tab) - 1:
        if tab[x][y] > tab[x+1][y]:
            return False
    if y > 0: 
        if tab[x][y] > tab[x][y-1]:
            return False
    if y < len(tab[0]) - 1:
        if tab[x][y] > tab[x][y+1]:
            return False
    return True

data = [[0 for x in range(len(inputs[0]))] for y in range(len(inputs))]
for i in range(len(data)):
    for j, char in enumerate(inputs[i]):
        data[i][j]=int(char)

res = 0 
for i in range(len(data)):
    for j, char in enumerate(data[0]):
        if isLowPoint(data, i, j):
            res += int(data[i][j]) + 1

print(res)

