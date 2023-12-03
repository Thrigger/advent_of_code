#inputs = open("../inputs/examples/d03.txt").read().strip().split("\n")
inputs = open("../inputs/d03.txt").read().strip().split("\n")

def is_symbol(row, col):
    return schem[row][col] != '.' and not schem[row][col].isnumeric()

def neig_is_symbol(row, col):
    lr = max(row - 1, 0)
    hr = min(row + 1, len(schem) - 1)
    lc = max(col - 1, 0)
    hc = min(col + 1, len(schem[0]) - 1)

    for r in range(lr, hr + 1):
        for c in range(lc, hc + 1):
            if is_symbol(r, c):
                return True
    return False

schem = []
for line in inputs:
    schem.append(list(line))

res = 0
for row in range(len(schem)):
    keep_val = False
    val = ""
    for col in range(len(schem[0])):
        if schem[row][col].isnumeric():
            val += schem[row][col]
            if neig_is_symbol(row, col):
                keep_val = True
            if col == len(schem[0]) - 1 and keep_val:
                res += int(val)
        else:
            if keep_val:
                res += int(val)
            val = ""
            keep_val = False

print(res)

