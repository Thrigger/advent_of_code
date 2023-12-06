#inputs = open("../inputs/examples/d06.txt").read().strip().split("\n")
inputs = open("../inputs/d06.txt").read().strip().split("\n")

res = 1
_, times = inputs[0]. split(":")
times = times.strip().split()
_, dists = inputs[1]. split(":")
dists = dists.strip().split()

for i in range(len(dists)):
    time = int(times[i])
    dist = int(dists[i])
    count = 0
    for speed in range(time):
        # Speed is same as waiting time
        points = speed * (time -speed)
        if points > dist:
            count += 1
    res *= count

print(res)
