from collections import Counter
from functools import lru_cache

inputs = open("../inputs/d14.txt").read().strip().split("\n\n")

tmpl = list(inputs[0])
data = list(inputs[1].split("\n"))
rules = {}
for rule in data:
    ins = rule.split(" -> ") 
    rules[ins[0]] = ins[1]

@lru_cache(maxsize=None)
def count_sub(a, b, steps_left):
    if steps_left == 0:
        return Counter("")
    middle = rules[a+b]
    sub_par1 = count_sub(a, middle, steps_left-1)
    sub_par2 = count_sub(middle, b, steps_left-1)

    return Counter(middle)+sub_par1+sub_par2

count=Counter(tmpl)
for i in range(len(tmpl)-1):
    #count += count_sub(tmpl[i], tmpl[i+1], 10)
    count += count_sub(tmpl[i], tmpl[i+1], 40)

print(count)
print(max(count.values())-min(count.values()))

