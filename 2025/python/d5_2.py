#inputs = open("../inputs/examples/d05.txt").read().strip().split("\n\n")
inputs = open("../inputs/d05.txt").read().strip().split("\n\n")

res = 0
fresh = []
for line in inputs[0].split("\n"):
    start, stop = line.split("-")
    start = int(start)
    stop = int(stop)

    not_done = True
    while not_done:
        not_done = False
        # Remove contained
        i = 0
        while i < len(fresh):
            if fresh[i][0] >= start and fresh[i][1] <= stop:
                print("Removing")
                print(fresh[i])
                print(" "+str(start) + ", " + str(stop))
                print(fresh.pop(i))
            else:
                i+=1

        # Merge
        i = 0
        while i <len(fresh):
            if start >= fresh[i][0] and start <= fresh[i][1]:
                new = fresh.pop(i)
                start = new[0]
                if new[1] > stop:
                    stop = new[1]
                not_done = True
                break
            i+=1
        # Merge
        i = 0
        while i <len(fresh):
            if stop >= fresh[i][0] and stop <= fresh[i][1]:
                new = fresh.pop(i)
                stop = new[1]
                if new[0] < start:
                    start = new[0]
                not_done = True
                break
            i+=1
    fresh.append([int(start), int(stop)])

for f in fresh:
    res += f[1]-f[0]+1

print(res)
