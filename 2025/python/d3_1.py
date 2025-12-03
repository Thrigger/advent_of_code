#inputs = open("../inputs/examples/d03.txt").read().strip().split("\n")
inputs = open("../inputs/d03.txt").read().strip().split("\n")

def find2(bank):
    j = 9
    while j > 0:
        pos = bank.find(str(j))
        if pos >= 0 and pos != len(bank):
            return j
        j-=1
    print("ERROR")
    return -1

all_banks = []
for line in inputs:
    i = 9
    while i > 0:
        pos = line.find(str(i))
        if pos >= 0 and pos < len(line)-1:
            _first, second = line.split(str(i), 1);
            j = find2(second)
            all_banks.append(i*10+j)
            break
        i -= 1

print(sum(all_banks))
