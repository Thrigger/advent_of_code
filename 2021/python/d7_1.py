inputs = open("../inputs/d07.txt").read().strip().split(",")

data = []
for each in inputs:
    data.append(int(each))

data.sort()

max_fuel=-1
mean_value=-1
for i in range(min(data), max(data)):
    fuel = 0
    for each in data:
        fuel += abs(each - i)
    if fuel < max_fuel or max_fuel == -1:
        max_fuel = fuel
        mean_val = i

print(max_fuel)

