#inputs = open("../inputs/examples/d02.txt").read().strip().split("\n")
inputs = open("../inputs/d02.txt").read().strip().split("\n")

res = 0
for line in inputs:
    game_id, hands = line.split(": ")
    _, game_id = game_id.split(" ")
    hands = hands.split("; ")
    ok = True
    red=0
    green=0
    blue=0

    for hand in hands:
        types = hand.split(", ")
        for each in types:
            numb, color = each.split(" ")
            numb = int(numb)
            if color == "red":
                if numb > red:
                    red = numb
            elif color == "green":
                if numb > green:
                    green = numb
            elif color == "blue":
                if numb > blue:
                    blue = numb
    if ok:
        res += int(red*green*blue)

print(res)

