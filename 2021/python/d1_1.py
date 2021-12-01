
res=0
start = 0
for each in open("../inputs/d01.txt"):
    each = each.strip()
    if each == "":
        break
    print(each)
    if start != 0 and int(each) > start:
        res += 1
    start = int(each)

print(res)
