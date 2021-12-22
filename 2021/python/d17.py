inputs = open("../inputs/d17.txt").read().strip().split(": ")

(x,y) = inputs[1].split(", ")

x = x.split("=")
x = list(map(int, x[1].split("..")))
y = y.split("=")
y = list(map(int, y[1].split("..")))

def x_steps_to_goal(speed, xmin, xmax):
    end = int((speed*(speed+1))/2)
    if end < xmin:
        return 0
    current_x=0
    valid_steps=[]
    for step in range(1,xmax):
        current_x += speed
        if speed > 0:
            speed -= 1

        if current_x > xmax:
            if len(valid_steps) == 0:
                return 0
            else:
                return valid_steps
        elif current_x >= xmin:
            valid_steps.append(step)
    return valid_steps
    
def calc_y_end(speed, steps):
    end_pos = 0
    while steps > 0:
        steps -= 1
        end_pos += speed
        speed -= 1
    return end_pos
    
hits = 0
for current_x in range(x[1]+1):
    x_steps = x_steps_to_goal(current_x, x[0], x[1])
    if x_steps != 0:
        used_y=[]
        for steps in x_steps:
            for j in range(y[0], y[0] * -1):
                if j not in used_y:
                    y_landing = calc_y_end(j, steps)
                    if y_landing >= y[0] and y_landing <= y[1]:
                        used_y.append(j)
                        hits += 1
            
print(hits)

