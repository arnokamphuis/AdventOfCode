
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day03-{runtype}.txt", "r")

lines = [list(map(int, list(line.strip()))) for line in text_file.readlines()]

def find_jolts(line, digits):
    jolts = 0
    for i in range(digits-1, -1, -1):
        max_val = max(line[:len(line)-i])
        jolts = jolts * 10 + max_val
        line = line[line.index(max_val)+1:]
    return jolts

def part1():
    total = 0
    for line in lines:
        total += find_jolts(line, 2)
    return total

def part2():
    total = 0
    for line in lines:
        total += find_jolts(line, 12)
    return total

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
