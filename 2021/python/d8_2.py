inputs = open("../inputs/d08.txt").read().strip().split("\n")

data = []
for each in inputs:
    data.append(each.split(" | "))

res = 0 

def identify(dig):
    if len(dig) == 2:
        return 1
    elif len(dig) == 4:
        return 4
    elif len(dig) == 3:
        return 7
    elif len(dig) == 7:
        return 8
    else:
        return 0
 
def containSameParts(sub, main):
    result = True
    for each in sub:
        if main.count(each) == 0:
            result = False
            break
    return result

def idNumb(dec, numb):
    i = 0
    while i < len(dec):
        is_numb=True
        for char in numb:
            if dec[i].count(char) == 0:
                is_numb = False
                break
        for char in dec[i]:
            if numb.count(char) == 0:
                is_numb = False
                break
        if is_numb:
            break
        else:
            i+=1
    return i

for each in data:
    ins = each[0].split(" ")
    outs = each[1].split(" ")

    decoded = [""] * 10
    i = 0
    while i < len(ins):
        if identify(ins[i]) == 1:
            decoded[1] = ins[i] 
            ins.pop(i)
        elif identify(ins[i]) == 4:
            decoded[4] = ins[i] 
            ins.pop(i)
        elif identify(ins[i]) == 7:
            decoded[7] = ins[i] 
            ins.pop(i)
        elif identify(ins[i]) == 8:
            decoded[8] = ins[i] 
            ins.pop(i)
        else:
            i+=1
            
    i = 0
    while i < len(ins):
        if len(ins[i]) == 6:
            if containSameParts(decoded[4], ins[i]):
                decoded[9] = ins[i]
            elif containSameParts(decoded[7], ins[i]):
                decoded[0] = ins[i]
            else:
                decoded[6] = ins[i]
            ins.remove(ins[i])
        else:
            i += 1

    i = 0
    while i < len(ins):
        if len(ins[i]) == 5:
            if containSameParts(decoded[1], ins[i]):
                decoded[3] = ins[i]
            elif containSameParts(ins[i], decoded[6]):
                decoded[5] = ins[i]
            else:
                decoded[2] = ins[i]
            ins.remove(ins[i])

    out_val=""
    for out in outs:
        out_val += str(idNumb(decoded, out))

    res += int(out_val)
print(res)

