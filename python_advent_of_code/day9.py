

with open("../files/day9.txt", "r") as file:
    moves = [[line.split(" ")[0], int(line.split(" ")[1])] for line in file.read().split("\n")]

motionVectors = {'L': (-1, 0), 'R': (1, 0), 'U': (0, -1), 'D': (0, 1)}

xH, yH = 0, 0
xT, yT = 0, 0
visitedByTail = set()
for direction, amount in moves:
    mx, my = motionVectors[direction]
    for i in range(amount):
        xH, yH = xH+mx, yH+my
        dx, dy = xH-xT, yH-yT
        if abs(dx) > 1 or abs(dy) > 1:
            xT = xT+dx//2 if abs(dx)>1 else xT+dx
            yT = yT+dy//2 if abs(dy)>1 else yT+dy
        visitedByTail.add((xT, yT))

print(len(visitedByTail))

rope = [[0, 0] for i in range(10)]
visitedByTail = set()
for direction, amount in moves:
    for j in range(amount):
        mx, my = motionVectors[direction]
        for i in range(10):
            rope[i][0], rope[i][1] = rope[i][0]+mx, rope[i][1]+my
            if i < 10-1:
                dx, dy = rope[i][0]-rope[i+1][0], rope[i][1]-rope[i+1][1]
                if abs(dx) > 1 or abs(dy) > 1:
                    mx = dx//2 if abs(dx) > 1 else dx
                    my = dy//2 if abs(dy) > 1 else dy
                else:
                    mx, my = 0, 0
        visitedByTail.add(tuple(rope[-1]))

print(len(visitedByTail))