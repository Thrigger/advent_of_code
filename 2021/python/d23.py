# \n
inputs = open("../inputs/examples/d23.txt").read().strip().split("\n")
#inputs = open("../inputs/examples/d23_complete.txt").read().strip().split("\n")
#inputs = open("../inputs/d23.txt").read().strip().split("\n")
# ,
#inputs = open("../inputs/examples/d23.txt").read().strip().split(",")
#inputs = open("../inputs/d23.txt").read().strip().split(",")


def print_cave(data):
    for each in data:
        print("".join(each))

data = []
for each in inputs:
    data.append(list(each))

def is_sorted(data):
    i = 0 
    while i < len(data[2]):
        last = None
        if data[2][i] != "#":
            current = data[2][i]
            if last != None and last > current:
                return False
            if current != data[3][i]:
                return False
            last = current
        i+=1
    return True

def is_half_complete(data):
    if data[3][9] == "D" and data[2][9] != "D":
        return "D"
    elif data[3][7] == "C" and data[2][7] != "C":
        return "C"
    elif data[3][5] == "B" and data[2][5] != "B":
        return "B"
    elif data[3][3] == "A" and data[2][3] != "A":
        return "A"
    return None

def get_next_fish_to_move(data):
    #Någon som redan rör sig
    fish = is_half_complete(data)
    if fish != None:
        return fish
    #ta dyraste

def is_in_correct_place(data, x, y):
    fish = data[y][x];
    if y == 1:
        return False
    elif y == 2:
        if fish != data[y+1][x]:
            return False
        if x == 3 and fish == "A":
            return True
        elif x == 5 and fish == "B":
            return True
        elif x == 7 and fish == "C":
            return True
        elif x == 9 and fish == "D":
            return True
    elif y == 3:
        if x == 3 and fish == "A":
            return True
        elif x == 5 and fish == "B":
            return True
        elif x == 7 and fish == "C":
            return True
        elif x == 9 and fish == "D":
            return True
    return False

# This functionw ill only retrun those not allready in home
def get_all_fishes_of_type(data, fish_type):
    x=0
    fishes=[]
    while x < 4:
        y=0
        while y < 2:
            if data[y+2][3+2*x] == fish_type:
                print(str(2*x+3)+","+str(2+y))
                if not(is_in_correct_place(data, 2*x+3, 2+y)):
                    fishes.append((3+2*x,2+y))
            y+=1
        x+=1
    x = 0
    for each in data[1]:
        if each == fish_type:
            fishes.append((x,1))
        x+=1
    return fishes

def get_target(data, fish):
    if fish == "A":
        x = 3
    elif fish == "B":
        x = 5
    elif fish == "C":
        x = 7
    elif fish == "D":
        x = 9

    for y in range(3, 0, -1):
        if data[y][x] != fish:
            return (x,y)
    print("ERROR")

def can_move_out(data, x, y, tar):
    for j in range(2,y):
        if data[j][x] != ".":
            return False
    return True

def can_move_in(data, x, y, tar):
    for j in range(2,tar[1]+1):
        if data[j][tar[0]] != ".":
            return False
    return True
        
def can_move_top_row(data, x, y, tar):
    delta = 1
    if tar[0] < x:
        delta = -1
    for i in range(x, tar[0], delta):
        if data[1][i] != ".":
            return False
    return True

def get_x_for_fish_home(fish):
    if fish == "A":
        return 3
    elif fish == "B":
        return 5
    elif fish == "C":
        return 7
    elif fish == "D":
        return 9

def can_move_to_target(data, x, y, tar):
    if not can_move_top_row(data, x, y, tar):
        return False
    if not can_move_out(data, x, y, tar):
        return False
    if not can_move_in(data, x, y, tar):
        return False
    return True

def move_top(data, x, y, tar):
    delta = 1
    if tar[0] < x:
        delta = -1
    for i in range(x, tar[0], delta):
        if data[1][i] != ".":
            blocking = [i, 1, data[1][i]]
            break
    
    blocking_target = get_x_for_fish_home(blocking[2])
    if 
    #TODO can be correctly placed?



    return data

def clean_path(data, x, y, tar):
    if not can_move_top_row(data, x, y, tar):
        data = move_top(data, x, y, tar)
    if not can_move_out(data, x, y, tar):
        return False
    if not can_move_in(data, x, y, tar):
        return False


energy = 0
while True:
    print_cave(data)
    print(is_sorted(data))
    if is_sorted(data):
        break
    fish_type_to_move = get_next_fish_to_move(data)
    print(fish_type_to_move)
    fishes = get_all_fishes_of_type(data, fish_type_to_move)
    print(fishes)
    #TODO sort fishes?
    for (x,y) in fishes:
        target=get_target(data, data[y][x])
        print(target)
        
        while not(can_move_to_target(data, x, y, target)):
            print("path is not clean")
            (data,new_energy)=clean_path(data, x, y, target)
            energy+=new_energy
            break



    tmp = input()

print(energy)

