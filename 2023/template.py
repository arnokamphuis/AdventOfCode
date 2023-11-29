
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])

text_file = open("dayxx-{}.txt".format(runtype), "r")

lines = [line.split() for line in text_file.readlines()]

def part1():
    return 0

def part2():
    return 0

if runpart == 1 or runpart == 0:
    print("Part 1: {}".format(part1()))

if runpart == 2 or runpart == 0:
    print("Part 2: {}".format(part2()))
    
