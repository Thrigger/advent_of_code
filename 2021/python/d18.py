# \n
#inputs = open("../inputs/examples/d18.txt").read().strip().split("\n")
#inputs = open("../inputs/examples/d18_3.txt").read().strip().split("\n")
inputs = open("../inputs/d18.txt").read().strip().split("\n")
# ,
#inputs = open("../inputs/examples/d18.txt").read().strip().split(",")
#inputs = open("../inputs/d18.txt").read().strip().split(",")

numbers = []
for each in inputs:
    numbers.append(list(each))

def get_max_dept(number):
    dep = 0
    max_dep = 0
    for each in number:
        if each == "[":
            dep += 1
            if dep > max_dep:
                max_dep = dep
        elif each == "]":
            dep -= 1
    return max_dep

def get_largest_val(number):
    result = 0
    for each in number:
        if each != "[" and each != "]" and each != ",":
            if int(each) > result:
                result = int(each)
    return result

def explode(number, offset):
    left = number[offset]
    j = offset - 1
    while j >= 0:
        if number[j] not in ["[", "]", ","]:
            number[j] = str(int(number[j]) + int(left))
            break
        j -= 1
    right = number[offset+2]
    j = offset + 3
    while j < len(number):
        if number[j] not in ["[", "]", ","]:
            number[j] = str(int(number[j]) + int(right))
            break
        j += 1
    del number[offset-1:offset+3]
    number[offset-1] = "0"

def explode_first(number):
    dep = 0
    max_dep = 0
    result = []
    i = 0 
    while i < len(number):
        if number[i] == "[":
            dep += 1
            if dep > max_dep:
                max_dep = dep
                if max_dep >= 5:
                    explode(number, i+1)
                    break
        elif number[i] == "]":
            dep -= 1
        i+=1
    return number

def split_large(number):
    i = 0
    while i < len(number):
        each = number[i]
        if each != "[" and each != "]" and each != ",":
            val = int(number[i]) 
            if val >= 10:
                left = int(val/2)
                right = int((val + 1)/2)
                number[i]="]"
                number.insert(i, str(right))
                number.insert(i, ",")
                number.insert(i, str(left))
                number.insert(i, "[")
                break
        i+=1
    return number

def add_numbers(numbers):
    result = []
    for number in numbers:
        if len(result)==0:
            result = number
        else:
            result = ["["]+result+[","]+number+["]"]

        while True:
            dep = get_max_dept(result)
            largest = get_largest_val(result)
            #print("working: "+"".join(result))
            #print("dep: "+str(dep))
            #print("Largest numb: "+str(largest))

            if dep >= 5:
                #Explode
                result = explode_first(result)
            elif largest >= 10:
                #split
                result = split_large(result)
            else:
                break
        #print("after reduction: "+"".join(result))
    return result

def calc_sum(l,r):
    return 3*int(l) + 2*int(r)

def extract_first(number):
    i = 0
    while i < len(number):
        each = number[i]
        if each != "[" and each != "]" and each != ",":
            each2 = number[i+2]
            if each2 != "[" and each2 != "]" and each2 != ",":
                new_val = calc_sum(number[i], number[i+2])
                del number[i-1:i+4]
                number.insert(i-1, new_val)
                break
        i+=1
    return number


def calc_mag(result):
    while True:
        if len(result) == 1:
            return result[0]
            break
        else:
            result=extract_first(result)

print("Part 1")
res = add_numbers(numbers.copy())
print(calc_mag(res))

max_mag = 0
i = 0
while i < len(numbers):
    j = 0
    while j < len(numbers):
        numbs = []
        numbs.append(numbers[i].copy())
        numbs.append(numbers[j].copy())
        if j != i:
            res = add_numbers(numbs)
            mag = calc_mag(res)
            if mag > max_mag:
                max_mag = mag
        j+=1
    i+=1

print("Part 2")
print(max_mag)




