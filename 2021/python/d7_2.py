inputs = open("../inputs/d07.txt").read().strip().split(",")

data = []
for each in inputs:
    data.append(int(each))

data.sort()

max_fuel=-1
for i in range(min(data), max(data)):
    fuel = 0
    last_fuel = -1
    last_val = -1
    for each in data:
        if each == last_val:
            current_fuel = last_fuel
        else:
            cost = 1
            current_fuel = 0
            for step in range(abs(each-i)):
                current_fuel += cost
                cost += 1
            last_fuel = current_fuel
            last_val = each
        fuel += current_fuel

    if fuel < max_fuel or max_fuel == -1:
        index = i
        max_fuel = fuel
    elif fuel > max_fuel:
        break

print(max_fuel)

