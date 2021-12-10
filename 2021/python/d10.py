inputs = open("../inputs/d10.txt").read().strip().split("\n")

data = []
for each in inputs:
    data.append(list(each))

part1 = 0
incorrect=[]
for row in data:
    stack = []
    corrupt=False
    for each in row:
        if each in ['(', '{', '[', '<']:
            stack.append(each)
        else:
            tmp = stack.pop()
            if (tmp == '(' and not(each == ')')) or (tmp == '<' and not(each == '>')) or (tmp == '[' and not(each == ']')) or (tmp == '{' and not(each == '}')):
                corrupt=True
                if each == ')':
                    part1 += 3
                elif each == ']':
                    part1 += 57
                elif each == '}':
                    part1 += 1197
                elif each == '>':
                    part1 += 25137
                break
    if not(corrupt):
        incorrect.append(stack)

print(part1)

points = []
for row in incorrect:
    tmp = 0
    for char in reversed(row):
        tmp *= 5
        if char == '(':
            tmp += 1
        elif char == '[':
            tmp += 2
        elif char == '{':
            tmp += 3
        elif char == '<':
            tmp += 4
    points.append(tmp)

points.sort()
print(points[(len(points))/2])


