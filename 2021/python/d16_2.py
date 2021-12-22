inputs = open("../inputs/d16.txt").read().strip()

data = ""
for each in inputs:
    data += "{0:04b}".format(int(each,16))

def calc_lit():
    global data
    run = 1
    result = ""
    while run==1:
        run = int(data[:1])
        data=data[1:]
        result += data[:4]
        data = data[4:]
    return int(result, 2)

def calc_subpackets(type_id, sub_packets):
    global data
    val = 0
    if type_id == 0:
        while sub_packets > 0:
            sub_packets -= 1
            val += calc_packet()
    elif type_id == 1:
        val = 1
        while sub_packets > 0:
            sub_packets -= 1
            val *= calc_packet()
    elif type_id == 2:
        tmp= []
        while sub_packets > 0:
            sub_packets -= 1
            tmp.append(calc_packet())
        val = min(tmp)
    elif type_id == 3:
        tmp= []
        while sub_packets > 0:
            sub_packets -= 1
            tmp.append(calc_packet())
        val = max(tmp)
    elif type_id == 5:
        a=calc_packet()
        b=calc_packet()
        if a > b:
            val = 1
        else:
            val = 0
    elif type_id == 6:
        a=calc_packet()
        b=calc_packet()
        if a < b:
            val = 1
        else:
            val = 0
    elif type_id == 7:
        a=calc_packet()
        b=calc_packet()
        if a == b:
            val = 1
        else:
            val = 0
    return val
            
def calc_fixed_len(t_id, leng):
    global data
    start_len = len(data)
    val = 0
    if t_id == 0:
        while len(data)+leng >start_len:
            val += calc_packet()
    elif t_id == 1:
        val = 1
        while len(data)+leng >start_len:
            val *= calc_packet()
    elif t_id == 2:
        tmp =[]
        while len(data)+leng >start_len:
            tmp.append(calc_packet())
        val = min(tmp)
    elif t_id == 3:
        tmp =[]
        while len(data)+leng >start_len:
            tmp.append(calc_packet())
        val = max(tmp)
    elif t_id == 5:
        a=calc_packet()
        b=calc_packet()
        if a > b:
            val = 1
        else:
            val = 0
    elif t_id == 6:
        a=calc_packet()
        b=calc_packet()
        if a < b:
            val = 1
        else:
            val = 0
    elif t_id == 7:
        a=calc_packet()
        b=calc_packet()
        if a == b:
            val = 1
        else:
            val = 0
    return val
            
def calc_packet():
    global data

    if data.count("0") == len(data):
        return 0
    version = int(data[:3],2)
    data = data[3:]
    type_id = int(data[:3],2)
    data = data[3:]
    if type_id == 4:
        return calc_lit()
    else:
        len_id = data[:1]
        data=data[1:]
        if len_id == "0":
            sub_len = int(data[:15],2)
            data = data[15:]
            val = calc_fixed_len(type_id, sub_len)
            return val

        else:
            sub_packets = int(data[:11],2)
            data = data[11:]
            val = calc_subpackets(type_id, sub_packets)               
            return val

    return 0

print(calc_packet())

