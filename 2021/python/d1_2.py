
res=0
last = 0
index = 0
lines =  open("../inputs/d01.txt").readlines()
while index + 2 <= len(lines):
    if lines[index].strip() == "":
        break
    
    # get current
    if lines[index+2].strip() != "":
        current = int(lines[index].strip()) + int(lines[index+1].strip()) + int(lines[index+2].strip())
    else:
        break

    if last != 0 and current > last:
        res += 1
    last = current

    index += 1

print(res)
