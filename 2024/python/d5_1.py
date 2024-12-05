#inputs = open("../inputs/examples/d05.txt").read().strip().split("\n\n")
inputs = open("../inputs/d05.txt").read().strip().split("\n\n")

res = 0
rules = {}
for line in inputs[0].split("\n"):
    parts = line.split("|")
    if parts[0] in rules:
        rules[parts[0]].append(parts[1])
    else:
        rules[parts[0]] = [parts[1]]

updates = []
for line in inputs[1].split("\n"):
    updates.append(line)

for up in updates:
    numbs =list(up.split(","))
    i = 0
    while i < len(numbs):
        if numbs[i] in rules:
            sub_rules = rules[numbs[i]]
            before = numbs[0:i]
            if len(set(before) & set(sub_rules)) > 0:
                break
        i += 1
        if i == len(numbs):
            res += int(numbs[int(i/2)])
        
print(res)
