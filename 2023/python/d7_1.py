#inputs = open("../inputs/examples/d07.txt").read().strip().split("\n")
inputs = open("../inputs/d07.txt").read().strip().split("\n")

def get_values(cards):
    ret = []
    for each in cards:
        if each == "A":
            ret.append(14)
        elif each == "K":
            ret.append(13)
        elif each == "Q":
            ret.append(12)
        elif each == "J":
            ret.append(11)
        elif each == "T":
            ret.append(10)
        else:
            ret.append(int(each))
    return ret

def get_points(hand):
    same = []
    for each in hand:
        same.append(hand.count(each))
    same = sorted(same, reverse=True)

    if same[0] == 5:
        return 6
    elif same[0] == 4:
        return 5
    elif same[0] == 3 and same[3] == 2:
        return 4
    elif same[0] == 3:
        return 3
    elif same[0] == 2 and same[2] == 2:
        return 2
    elif same[0] == 2:
        return 1
    return 0

def is_new_higher(old, new):
    i = 0 
    while i < len(old):
        if old[i] > new[i]:
            return False
        elif old[i] < new[i]:
            return True
        i += 1

def larger_than(old, new):
    o_points = get_points(old)
    n_points = get_points(new)
    if n_points > o_points or (n_points == o_points and is_new_higher(old, new)) :
        return True
    return False

hands = []
for line in inputs:
    cards, bet = line.split() 
    cards = get_values(cards)
    hands.append((cards, bet))

sorted_hands = []
res = 0
i = 0
while i < len(hands):
    hand = hands[i]
    for j in range(len(sorted_hands) + 1):
        if j == len(sorted_hands):
            sorted_hands.append(hand)
            break
        else:
            old, _ = sorted_hands[j]
            new, _ = hand
            if not larger_than(old,new):
                sorted_hands.insert(j, hand)
                break
    hands.remove(hand)

for i in range(len(sorted_hands)):
    hand = sorted_hands[i]
    _, bet = hand
    res += int(bet) * (1+i)

print(res)
