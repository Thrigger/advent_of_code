#inputs = open("../inputs/examples/d07.txt").read().strip().split("\n")
inputs = open("../inputs/d07.txt").read().strip().split("\n")

size = len(inputs)
def equal_to_goal(goal, vs):
    if len(vs) == 1 and goal == vs[0]:
        return True
    elif len(vs) == 1:
        return False

    a = [vs[0]+vs[1]]
    m = [vs[0]*vs[1]]
    c = [int(str(vs[0])+str(vs[1]))]

    if len(vs) > 2:
        a += vs[2:]
        m += vs[2:]
        c += vs[2:]
    return equal_to_goal(goal, a) or equal_to_goal(goal, m) or equal_to_goal(goal, c)
    
res = 0
for line in inputs:
    (ex, vals) = line.split(": ")
    ex = int(ex)
    vals = [int(s) for s in vals.split(" ")]

    if equal_to_goal(ex, vals):
        res += ex

print(res)
