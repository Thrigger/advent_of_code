inputs = open("../inputs/examples/d12.txt").read().strip().split("\n")
#inputs = open("../inputs/d12.txt").read().strip().split("\n")

def get_possible(rec, numbs):
    print("in possible check")
    poss = []

    print("Rec - numbs")
    print(rec)
    print(numbs)
    while True:
        done, rec, numbs =  simplify(rec, numbs)
        print("Rec - numbs")
        print(rec)
        print(numbs)
        if done:
            break

    return poss

def simplify(rec, numbs):
    done = False
    old = rec
    rec = rec.strip(".")
    rec, numbs = trim_done(rec, numbs)
    if old == rec:
        done = True
    return done, rec, numbs

def trim_done(rec, numbs):
    if rec[0] == "#":
        val = numbs.pop(0)
        rec = rec[val:]
    if rec[-1] == "#":
        val = numbs.pop(-1)
        rec = rec[:-val]
    return rec, numbs


res = 0
for line in inputs:
    record, numbs = line.split()
    numbs = [ int(x) for x in numbs.split(",") ]
    poss = get_possible(record, numbs)
    #print(poss)
    res += len(poss)

print(res)

#              function     itterator
#map(lambda x: int(x), line.split("\n"))
#vec = line.split(" ")
#vec = list(b)
#vec.append(a)
#len(vec)
#sorted(vec)   <- small to big
#print(vec)
#print(vec[0])
#list(set(some_list) & set(some_other_list))    <-- check overlapping

