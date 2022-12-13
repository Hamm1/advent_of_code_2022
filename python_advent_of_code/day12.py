

with open("../files/day12.txt", "r") as file:
    lines = file.read().split("\n")
    xMax, yMax = len(lines[0]), len(lines)
    startingPoints = []
    heights = [[0 for x in range(xMax)] for y in range(yMax)]
    for y, line in enumerate(lines):
        for x, letter in enumerate(line):
            if letter == "S":
                start = (x, y)
                heights[y][x] = 0
            elif letter == "E":
                end = (x, y)
                heights[y][x] = 25
            elif letter == "a":
                startingPoints.append((x, y))
                heights[y][x] = 0
            else:
                heights[y][x] = "abcdefghijklmnopqrstuvwxyz".find(letter)


def shortestPathLength(start, end):
    toDo = [(start, 0)]
    visited = set([start])
    directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    while toDo:
        (x, y), pathLength = toDo.pop()
        if (x, y) == end:
            return pathLength
        for dx, dy in directions:
            if x+dx in range(xMax) and y+dy in range(yMax) and (x+dx, y+dy) not in visited:
                if heights[y+dy][x+dx] <= heights[y][x]+1:
                    toDo.append(((x+dx, y+dy), pathLength+1))
                    visited.add((x+dx, y+dy))
        toDo.sort(key=lambda entry: -entry[1])
    return float("inf")


print(shortestPathLength(start, end))



def shortestPathLengthReverse(end):
    toDo = [(end, 0)]
    visited = set([end])
    directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    while toDo:
        (x, y), pathLength = toDo.pop()
        if heights[y][x] == 0:
            return pathLength
        for dx, dy in directions:
            if x+dx in range(xMax) and y+dy in range(yMax) and (x+dx, y+dy) not in visited:
                if heights[y][x] <= heights[y+dy][x+dx]+1:
                    toDo.append(((x+dx, y+dy), pathLength+1))
                    visited.add((x+dx, y+dy))
        toDo.sort(key=lambda entry: -entry[1])
    return float("inf")

print(shortestPathLengthReverse(end))

