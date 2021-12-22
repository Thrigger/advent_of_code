inputs = open("../inputs/d17.txt").read().strip().split(": ")

(x,y) = inputs[1].split(", ")

x = x.split("=")
x = list(map(int, x[1].split("..")))
y = y.split("=")
y = list(map(int, y[1].split("..")))

def x_landing(x):
    return int((x*(x+1))/2)


valid_x=[]
i = 0
while True:
    x_land = x_landing(i)
    if x_land >= x[0] and x_land <= x[1]:
        valid_x.append(i)
    elif x_land > x[1]:
        break
    i+= 1

def get_y_heigth(y, steps, ymin):
    res = 0
    top = 0
    while steps > 0 or ymin <= res+y:
        res += y
        y -= 1
        if res >= top:
            top = res
        steps -=1

    return (top, res)
        

best_elevation = 0
best_point = -1

for each in valid_x:
    for i in range((y[0] * -1)):
        (ytop, ylanding) = get_y_heigth(i, each, y[0])
        if ylanding >= y[0] and ylanding <= y[1]:
            if ytop >=  best_elevation:
                best_elevation = ytop
                best_point = (each, i)


print(ytop)
