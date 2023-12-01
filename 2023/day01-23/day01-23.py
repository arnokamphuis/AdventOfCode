
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])

text_file = open("day01-{}.txt".format(runtype), "r")

lines = [line.split() for line in text_file.readlines()]

def part1():
    res = 0
    for line in lines:
        number = []
        line = line[0]
        for i in range(len(line)):
            if line[i].isdigit():
                number.append(line[i])
        res += int(number[0] + number[-1])
    return res

def part2():
    res = 0
    numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
    for line in lines:
        number = []
        line = line[0]
        for i in range(len(line)):
            if line[i].isdigit():
                number.append(line[i])
            else:
                for ni in range(len(numbers)):
                    if line[i:].startswith(numbers[ni]):
                        number.append(str(ni+1))
        res += int(number[0] + number[-1])
    return res

if runpart == 1 or runpart == 0:
    print("Part 1: {}".format(part1()))

if runpart == 2 or runpart == 0:
    print("Part 2: {}".format(part2()))
    
