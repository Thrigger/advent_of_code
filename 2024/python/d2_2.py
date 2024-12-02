#inputs = open("../inputs/examples/d02.txt").read().strip().split("\n")
inputs = open("../inputs/d02.txt").read().strip().split("\n")

res = 0
for line in inputs:
    levels = line.split()
    for i in range(len(levels)):
        new_levels = levels.copy()
        del new_levels[i]
        dec = True
        inc = True
        last = 0
        for level in new_levels:
            level = int(level)
            if dec and inc and last == 0:
                last = level
            else:
                if inc and last > level:
                    inc = False
                if dec and last < level:
                    dec = False

                diff =  abs(last-level) 
                if diff < 1 or diff > 3:
                    inc = False
                    dec = False
                last = level
        if dec or inc:
            res += 1
            break
print(res)
