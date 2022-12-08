

with open("../files/day8.txt", "r") as file:
    heights = file.read().split("\n")

xMax, yMax = len(heights[0]), len(heights)

visible = set()
for y in range(yMax):
    maxHeight = heights[y][0]
    visible.add((y, 0))
    for x in range(xMax):
        if heights[y][x] > maxHeight:
            visible.add((y, x))
            maxHeight = heights[y][x]
    maxHeight = heights[y][-1]
    visible.add((y, xMax-1))
    for x in range(xMax-1, -1, -1):
        if heights[y][x] > maxHeight:
            visible.add((y, x))
            maxHeight = heights[y][x]

for x in range(xMax):
    maxHeight = heights[0][x]
    visible.add((0, x))
    for y in range(yMax):
        if heights[y][x] > maxHeight:
            visible.add((y, x))
            maxHeight = heights[y][x]
    maxHeight = heights[-1][x]
    visible.add((xMax-1, x))
    for y in range(yMax-1, -1, -1):
        if heights[y][x] > maxHeight:
            visible.add((y, x))
            maxHeight = heights[y][x]

print(len(visible))

