inputs = open("../inputs/d04.txt").read().strip().split("\n")

def print_chart(chart):
    print(chart[:5])
    print(chart[5:10])
    print(chart[10:15])
    print(chart[15:20])
    print(chart[20:])

def sum_chart(chart, num):
    res=0
    for each in chart:
        if each != 'xx':
            res += int(each)
    print(res * int(num))

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

total_charts=len(charts)
total_wins=0
for num in numbers:
    for i, chart in enumerate(charts):
        for j, char in enumerate(chart):
            if char == num:
                charts[i][j] = 'xx'

    #check for bingo 
    bingo = True
    for chart in charts:
        # rows
        for j in range(5):
            bingo = True
            for i in range(5):
                if chart[j*5 + i] != 'xx':
                    bingo = False
                    break
            if bingo:
                break
        if bingo:
            print("removed row")
            total_wins+=1
            print_chart(chart)
            charts.remove(chart)
            continue
        ##diag
        #bingo = True
        #for j in range(5):
        #    if chart[j+ j*5] != 'x':
        #        bingo = False
        #        break
        #if bingo:
        #    total_wins+=1
        #    print_chart(chart)
        #    charts.remove(chart)
        #    continue
        ##diag
        #bingo = True
        #for j in range(5):
        #    if chart[4 - j + j*5] != 'x':
        #        bingo = False
        #        break
        #if bingo:
        #    total_wins+=1
        #    print_chart(chart)
        #    charts.remove(chart)
        #    continue
        #cols
        for j in range(5):
            bingo = True
            for i in range(5):
                if chart[j+ i*5] != 'xx':
                    bingo = False
                    break
            if bingo:
                break
        if bingo:
            print("removed col")
            total_wins +=1
            print_chart(chart)
            charts.remove(chart)
            continue

    if total_wins == total_charts:
        print_chart(chart)
        sum_chart(chart, num)
        break

