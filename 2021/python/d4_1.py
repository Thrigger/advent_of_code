
# \n
#inputs = open("../inputs/examples/d04.txt").read().strip().split("\n")
#inputs = open("../inputs/d04.txt").read().strip().split("\n")
# ,
#inputs = open("../inputs/examples/d04.txt").read().strip().split("\n")
inputs = open("../inputs/d04.txt").read().strip().split("\n")


i = 0
charts = []
for puzz in inputs:
    if i == 0:
        numbers = puzz.split(",")
    elif len(charts) > 0 and len(charts[-1]) != 25:
        for each in puzz.split():
            charts[-1].append(each)

    else:
        charts.append(list(puzz))

    i+=1

print(numbers)
print(charts)

def sum_chart(chart, num):
    res=0
    for each in chart:
        if each != 'x':
            res += int(each)
    print(res * int(num))

for num in numbers:
    
    for i, chart in enumerate(charts):
        for j, char in enumerate(chart):
            if char == num:
                charts[i][j] = 'x'


    #check for bingo 
    bingo = True
    for i, chart in enumerate(charts):
        # rows
        for j in range(5):
            bingo = True
            for i in range(5):
                if chart[j*5 + i] != 'x':
                    bingo = False
                    break
            if bingo:
                break;
        if bingo:
            sum_chart(chart,num)
            break
        #cols
        for j in range(5):
            bingo = True
            for i in range(5):
                if chart[j+ i*5] != 'x':
                    bingo = False
                    break
            if bingo:
                break;
        if bingo:
            sum_chart(chart,num)
            break
        #diag
        bingo = True
        for j in range(5):
            if chart[j+ i*5] != 'x':
                bingo = False
                break
            elif chart[5 - j + i*5] != 'x':
                bingo = False

                break
        if bingo:
            sum_chart(chart,num)
            break
    if bingo:
        break


print(charts[2])

