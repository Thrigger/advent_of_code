inputs = open("../inputs/d06.txt").read().strip().split(",")

fiskar = []
for fisk in inputs:
    fiskar.append(int(fisk))

for _ in range(256):
    for i in range(len(fiskar)):
        fiskar[i] -= 1
        if fiskar[i] == -1:
            fiskar[i] = 6
            fiskar.append(8)
    
print(len(fiskar))

