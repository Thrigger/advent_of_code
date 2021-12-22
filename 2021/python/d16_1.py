inputs = open("../inputs/d16.txt").read().strip()

data = ""
for each in inputs:
    data += "{0:04b}".format(int(each,16))

def calc_packet(data):
    total_version=0
    while len(data)>0:
        if data.count("0") == len(data):
            break
        version = int(data[:3],2)
        total_version += version
        data = data[3:]
        type_id = int(data[:3],2)
        data = data[3:]
        val = 0
        if type_id == 4:
            run = 1
            result = ""
            while run==1:
                run = int(data[:1])
                data=data[1:]
                result += data[:4]
                data = data[4:]
            val = int(result, 2)
        else:
            len_id = data[:1]
            data=data[1:]
            if len_id == "0":
                sub_len = int(data[:15],2)
                data = data[15:]

            else:
                sub_packets = int(data[:11],2)
                data = data[11:]
    return total_version

print(calc_packet(data))

