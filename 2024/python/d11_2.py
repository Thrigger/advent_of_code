#inputs = open("../inputs/examples/d11.txt").read().strip().split("\n")
inputs = open("../inputs/d11.txt").read().strip().split("\n")

res = 0
stones = []
for line in inputs:
    stones = line.split(" ")

mem = {}
res_mem = {}
def get_number_of_stones_after(stone, turns):
    # End
    if turns == 0:
        return 1

    key = stone +"-"+str(turns)
    if key not in res_mem.keys():
        ret_val = 0

        # Solve subproblems
        new_stones = go_one_turn(stone)
        for each in new_stones:
            ret_val += get_number_of_stones_after(each, turns - 1)
        res_mem[key] = ret_val

    return res_mem[key]

def go_one_turn(stone):
    ret_val = []

    if stone == "0":
        ret_val += "1"
    elif len(stone) % 2 == 0:
        s2 = str(int(stone[int(len(stone)/2):]))
        s1 = stone[:int(len(stone)/2)]
        ret_val += [s1, s2]
    else:
        ret_val.append(str(int(stone) * 2024))
    return ret_val

for s in stones:
    val = get_number_of_stones_after(s, 75)
    res += val

print(res)
