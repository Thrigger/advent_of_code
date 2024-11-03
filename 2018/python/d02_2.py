#inputs = open("../inputs/examples/d02_2.txt").read().strip().split("\n")
inputs = open("../inputs/d02.txt").read().strip().split("\n")

res = ""
while len(inputs) > 0:
    current = inputs.pop()
    for each in inputs:
        wrongs = 0
        for i in range(len(each)):
            if each[i] != current[i]:
                wrongs += 1
        if wrongs == 1:
            for i in range(len(each)):
                if each[i] == current[i]:
                    res += each[i]

print(res)
