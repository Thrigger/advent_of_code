#inputs = open("../inputs/examples/d01.txt").read().strip().split("\n")
inputs = open("../inputs/d01.txt").read().strip().split("\n")

res = 0
for line in inputs:
    i=1
    numbs =["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
    for numb in numbs:
        if numb in line:
            # small hack to solve eightwo to eight + two
            line = line.replace(numb, numb + str(i) +numb)
        i+=1

    last = 0;
    first = 10;
    for each in line:
        if each.isnumeric():
            last = int(each)
            if first == 10:
                first = int(each)
    val = int(str(first)+str(last))
    res += val

print(res)

