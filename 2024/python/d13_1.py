#inputs = open("../inputs/examples/d13.txt").read().strip().split("\n\n")
inputs = open("../inputs/d13.txt").read().strip().split("\n\n")

def tup_sub(a,b):
    return (a[0]-b[0], a[1]-b[1])
def tup_add(a,b):
    return (a[0]+b[0], a[1]+b[1])
def tup_mul(t,a):
    return (a*t[0], a*t[1])
def tup_pos(t):
    return t[0] >= 0 and t[1] >= 0

def get_token_cost(a, b, left):
    ret_vals = []
    for i in range(101):
        a_toks = tup_mul(a, i) 
        if not tup_pos(tup_sub(left, a_toks)):
            break

        for j in range(101):
            toks = tup_add(a_toks, tup_mul(b, j))
            if not tup_pos(tup_sub(left, toks)):
                break
            if toks == left:
                ret_vals.append(i*3+j)
    return ret_vals
    

res = 0
for line in inputs:
    print("---------")
    vals = line.split("\n")
    a = tuple( [ int(s[2:]) for s in vals[0].split(": ")[1].split(", ") ] )
    b = tuple( [ int(s[2:]) for s in vals[1].split(": ")[1].split(", ") ] )
    p = tuple( [ int(s[2:]) for s in vals[2].split(": ")[1].split(", ") ] )
     
    cost = get_token_cost(a, b, p)
    if len(cost) > 0:
        res += min(cost)

print(res)
