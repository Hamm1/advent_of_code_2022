

with open("../files/day6.txt", "r") as file:
    signal = file.read()

for i in range(len(signal)):
    if len(set(signal[i:i+4])) == 4:
        break
print(i+4)


for i in range(len(signal)):
    if len(set(signal[i:i+14])) == 14:
        break
i+14

