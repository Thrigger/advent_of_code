inputs = open("../inputs/d03.txt").read().strip().split("\n")

res = ""
eps = ""
total_bits = 0
bits = []

for data in inputs:
    bits.append(data)
    total_bits += 1

rotatedList = []
for line in bits:
    i = 0
    for bit in line:
        if i >= len(rotatedList):
            rotatedList.append(bit)
        else:
            rotatedList[i] += bit
        i += 1

oxy = bits
co2 = list(oxy)
co2_rules = list(rotatedList)
i = 0
while i <= len(oxy[0]):
    if rotatedList[i].count("1") >= (1+len(rotatedList[0]))/2:
        mcb = "1"
    else:
        mcb = "0"

    oxy_index = 0
    #remove the wrong once
    while oxy_index < len(oxy):
        if oxy[oxy_index][i] != mcb:
            del oxy[oxy_index]
            #del rules also
            j = 0
            while j < len(rotatedList):
                rotatedList[j] = rotatedList[j][0:oxy_index]+rotatedList[j][oxy_index+1:]
                j +=1
        else:
            oxy_index+=1
    
    if len(oxy) == 1:
        break
    else:
        i += 1
 
i = 0
while i <= len(co2[0]):
    if co2_rules[i].count("1") >= (1+len(co2_rules[0]))/2:
        mcb = "0"
    else:
        mcb = "1"

    co2_index = 0
    #remove the wrong once
    while co2_index < len(co2):
        if co2[co2_index][i] != mcb:
            del co2[co2_index]
            #del rules also
            j = 0
            while j < len(co2_rules):
                co2_rules[j] = co2_rules[j][0:co2_index]+co2_rules[j][co2_index+1:]
                j +=1
        else:
            co2_index+=1
    
    if len(co2) == 1:
        break
    else:
        i += 1
    
print(int(co2[0],2)*int(oxy[0],2))
