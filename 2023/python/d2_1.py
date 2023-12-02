#inputs = open("../inputs/examples/d02.txt").read().strip().split("\n")
inputs = open("../inputs/d02.txt").read().strip().split("\n")

res = 0
for line in inputs:
    game_id, hands = line.split(": ")
    _, game_id = game_id.split(" ")
    hands= hands.split("; ")
    ok = True

    for hand in hands:
        types = hand.split(", ")
        for each in types:
            numb, color = each.split(" ")
            if color == "red":
                if int(numb) > 12:
                    ok = False
            elif color == "green":
                if int(numb) > 13:
                    ok = False
            elif color == "blue":
                if int(numb) > 14:
                    ok = False
    if ok:
        res += int(game_id)

print(res)

