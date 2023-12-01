
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])

text_file = open("day01-{}.txt".format(runtype), "r")

lines = [line.split() for line in text_file.readlines()]

def flatten_extend(matrix):
    flat_list = []
    for row in matrix:
        flat_list.extend(row)
    return flat_list

def find_value(line, revline):
    value = 0
    for c in line:
        if ord(c) >= ord('0') and ord(c) <= ord('9'):
            value += 10*int(c)
            break
    for c in revline:
        if ord(c) >= ord('0') and ord(c) <= ord('9'):
            value += int(c)
            break
    return value

def replace(line, sp, numbers):
    if line[sp[0]:sp[0]+len(numbers[sp[1]-1])] == numbers[sp[1]-1]:
        line = line[:sp[0]] + chr(ord('0')+sp[1]) + line[sp[0]+len(numbers[sp[1]-1]):]
    return line

def part1():
    res = 0
    for line in lines:
        res += find_value(line[0], line[0][::-1])
    return res

def part2():
    res = 0

    for line in lines:
        value = 0
        line = line[0]
        revline = line

        numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
        positions = flatten_extend(
            [[(i,p[0]) for i in p[1]] for p in list(
                filter(lambda x: len(x[1])>0, 
                       [(int(i)+1, [j for j in range(len(line)) if line.startswith(numbers[i], j)]) for i  in range(len(numbers))]
                )
            )]
        )
        sorted_positions = sorted(positions, key = lambda t: t[0])
        revsort_positions = sorted(positions, key = lambda t: t[0], reverse=True)

        for sp in sorted_positions:
            line = replace(line, sp, numbers)
        for sp in revsort_positions:
            revline = replace(revline, sp, numbers)
        value = find_value(line,revline[::-1])
        res += value
    return res

if runpart == 1 or runpart == 0:
    print("Part 1: {}".format(part1()))

if runpart == 2 or runpart == 0:
    print("Part 2: {}".format(part2()))
    
