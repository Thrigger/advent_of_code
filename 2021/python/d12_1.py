inputs = open("../inputs/d12.txt").read().strip().split("\n")

data = {}
for each in inputs:
    (start, end) = each.split("-")
    if not(data.has_key(start)):
        data[start] = []
    if not(data.has_key(end)):
        data[end] = []
    data[start].append(end)
    data[end].append(start)

routs=[]
def notVisited(visited, current):
    if current == "end":
        return True
    elif any(c for c in current if c.islower()):
        if current in visited:
            return False
    return True

def tryConnections(data, visited, current):
    visited.append(current)
    if current == "end":
        routs.append(visited)
        return

    for connection in data[current]:
        if notVisited(visited, connection):
            tryConnections(data, list(visited), connection)

tryConnections(data, [], "start")
print(len(routs))

