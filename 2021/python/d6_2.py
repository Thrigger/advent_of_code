inputs = open("../inputs/d06.txt").read().strip().split(",")

fiskar = [0] * 10
for fisk in inputs:
    fiskar[int(fisk)] += 1

for _ in range(256):
    for i in range(0,10):
        if i == 0:
            fiskar[9] = fiskar[i]
            fiskar[7] += fiskar[i]
        elif i == 9:
            fiskar[i-1] = fiskar[i]
            fiskar[i] = 0
        else:
            fiskar[i-1] = fiskar[i]
        
res = 0
for each in fiskar:
    res += each
print(res)

