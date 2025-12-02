#inputs = open("../inputs/examples/d02.txt").read().strip().split("\n")
inputs = open("../inputs/d02.txt").read().strip().split("\n")

def check_list(ns, val):
    sub_len = len(ns)//val
    for i in range(sub_len):
        for j in range(1,val):
            if ns[i] != ns[i+j*sub_len]:
                return False
    return True

def is_invalid(numb):
    n_list = list(str(numb))
    for m in range(2, 3):
        if len(n_list)%m == 0 :
            if check_list(n_list, m):
                return numb
    return 0

res = 0
for line in inputs:
    for seq in line.split(","):
        if seq == "":
            break
        start, stop = seq.split("-")
        for i in range(int(start), int(stop)+1):
            res += is_invalid(i)

print(res)
