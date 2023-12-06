#inputs = open("../inputs/examples/d06.txt").read().strip().split("\n")
inputs = open("../inputs/d06.txt").read().strip().split("\n")

res = 1
_, times = inputs[0]. split(":")
times = times.strip().replace(" ", "")
_, dists = inputs[1]. split(":")
dists = dists.strip().replace(" ", "")

time = int(times)
dist = int(dists)
for speed in range(time):
    points = speed * (time - speed)
    if points > dist:
        first = speed 
        break

for speed in range(time, 0, -1):
    points = speed * (time - speed)
    if points > dist:
        last = speed 
        break

print(last-first+1)

