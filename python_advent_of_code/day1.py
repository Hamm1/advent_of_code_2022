

import time
import sys

def main() :

    # Read/Clean the Input File to extract the Elves
    meals = []
    with open("../files/day1.txt", "r") as inputFile:
        # First Extract the Elves
        elves = [[int(calorie) for calorie in line.split("\n")] for line in inputFile.read().split("\n\n")]
        # Then collate those into meals and sort
        meals = sorted([sum(meals) for meals in elves], reverse=True)

    print(f"Part 1: {max(meals)}")
    print(f"Part 2: {sum(meals[0:3])}")

if __name__ == "__main__" :
    startTime = time.perf_counter()
    main()
    print(f"{(time.perf_counter() - startTime)}s", file=sys.stderr)
