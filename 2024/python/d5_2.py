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

broken = []
for up in updates:
    numbs =list(up.split(","))
    i = 0
    while i < len(numbs):
        if numbs[i] in rules:
            sub_rules = rules[numbs[i]]
            before = numbs[0:i]
            if len(set(before) & set(sub_rules)) > 0:
                broken.append(numbs)
                break
        i += 1

for broken_update in broken:
    fixed_update = []
    while len(broken_update) > 0:
        for numb in broken_update:
            other_numbs = set(broken_update)^set(numb)
            other_rules = []
            for o in other_numbs:
                if o in rules:
                    other_rules += rules[o]
            if numb not in other_rules:
                fixed_update.append(numb)
                broken_update.remove(numb)
                break
    res += int(fixed_update[int(len(fixed_update)/2)])

print(res)
