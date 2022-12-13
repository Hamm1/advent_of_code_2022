


with open("../files/day13.txt", "r") as file:
    pairs = [[eval(packet) for packet in pair.split("\n")] for pair in file.read().split("\n\n")]


def sign(x):
    return 0 if x==0 else 2*int(x>0)-1


def compare(left, right):
    if type(left) is int and type(right) is int:
        return sign(right-left)
    elif type(left) is list and type(right) is list:
        for l, r in zip(left, right):
            result = compare(l, r)
            if result != 0:
                return result
        return sign(len(right)-len(left))
    else:
        r = [right] if type(right) == int else right
        l = [left] if type(left) == int else left
        return compare(l, r)


sum(i+1 for i, (left, right) in enumerate(pairs) if compare(left,right) == 1)

