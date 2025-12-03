#inputs = open("../inputs/examples/d03.txt").read().strip().split("\n")
inputs = open("../inputs/d03.txt").read().strip().split("\n")

def find_largest(bank, val):
    i = 9

    while i > 0:
        pos = bank.find(str(i))
        if pos < 0:
            i-=1
        elif pos + 11 >= len(bank) + len(val):
            i-=1
        else:
            val +=  bank[pos]
            if len(val) == 12:
                return val
            return find_largest(bank[pos+1:], val)
    print("ERROR")
    return -1

all_banks = []
for line in inputs:
    val = ""
    val = find_largest(line, val)
    all_banks.append(val)

print(sum(map(int, all_banks)))
