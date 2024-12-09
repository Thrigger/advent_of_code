#inputs = open("../inputs/examples/d09.txt").read().strip().split("\n")
inputs = open("../inputs/d09.txt").read().strip().split("\n")

res = 0
i = 0
files = []
frees = []
for each in inputs[0]:
    each = int(each)
    if i % 2 == 0:
        files.append((int(i/2), each))
    else:
        files.append((-1, each))
    i += 1

i = 0
while i < len(files):
    (f_id, size) = files[i]
    if f_id == -1:
        if i == len(files):
            # Last
            break
        for j in list(reversed(range(i+1, len(files)))):
            if files[j][0] != -1 and files[j][1] <= size:
                files[i] = files[j]
                files[j] = (-1, files[i][1])
                size_diff = size - files[i][1]
                if size_diff > 0:
                    files.insert(i+1, (-1, size_diff))
                break
    i+=1

def get_check(index, fid, size):
    if size == 1:
        return index*fid
    return index * fid + get_check(index+1, fid, size-1)

i = 0 
real_i = 0
while i < len(files):
    if files[i][0] != -1:
        val = get_check(real_i, files[i][0], files[i][1])
        res += val
    real_i += files[i][1]
    i += 1
print(res)
