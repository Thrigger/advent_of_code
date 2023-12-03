#inputs = open("../inputs/examples/d03.txt").read().strip().split("\n")
inputs = open("../inputs/d03.txt").read().strip().split("\n")

def get_values_next_to(row, col):
    lr = max(row - 1, 0)
    hr = min(row + 1, len(schem)-1)
    lc = max(col - 1, 0)
    hc = min(col + 1, len(schem[0])-1)

    vals = []
    for r in range(lr, hr+1):
        ongoing_val = False
        for c in range(lc, hc+1):
            if schem[r][c].isnumeric():
                if not ongoing_val:
                    vals.append(get_val(r, c))
                ongoing_val = True
            else:
                ongoing_val = False
    return vals

def get_val(row, col):
    val = schem[row][col]
    c = col + 1
    while c < len(schem[0]):
        char = schem[row][c]
        if char.isnumeric():
            val = val + char
        else:
            break
        c += 1
    c = col - 1
    while c >= 0:
        char = schem[row][c]
        if char.isnumeric():
            val = char + val
        else:
            break
        c -= 1
    return int(val)

schem = []
for line in inputs:
    schem.append(list(line))

res = 0
for row in range(len(schem)):
    keep_val = False
    val = ""
    col = 0
    for col in range(len(schem[0])):
        if schem[row][col] == '*':
            vals=get_values_next_to(row, col)
            if len(vals) == 2:
                res += vals[0] * vals[1]

print(res)

