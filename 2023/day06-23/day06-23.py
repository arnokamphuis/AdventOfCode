
# read command-line parameters and based on that read the input file
import sys
from math import ceil, floor, sqrt, prod
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day06-{}.txt".format(runtype), "r")

lines = [line.split()[1:] for line in text_file.readlines()]
epsilon = 0.0000000001

def part1():
    data = [(int(t), int(d)) for t,d in list(zip(lines[0], lines[1]))]
    return prod([(
            int(((t+sqrt(t**2-4*(d+epsilon)))/2))-
            int(((t-sqrt(t**2-4*(d+epsilon)))/2))
        ) if t**2 > 4*(d+epsilon) else 0 for t,d in data])

def part2():
    t,d = [int("".join(lines[0])), int("".join(lines[1]))]
    sqrtD = sqrt(t**2-4*(d+epsilon))
    return int(((t+sqrtD)/2)) - int(((t-sqrtD))/2)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
