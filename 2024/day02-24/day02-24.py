
# read command-line parameters and based on that read the input file
import math
import sys
from typing import Counter
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1
    
sign = lambda x: math.copysign(1, x)

text_file = open(f"day02-{runtype}.txt", "r")

lines = [[int(x) for x in line.split()] for line in text_file.readlines()]

def check_safe(line, skip_one = False):
    if not skip_one:
        pairs = [(line[i], line[i + 1]) for i in range(len(line)-1)]
        return len(set([sign(p1-p2) for p1, p2 in pairs])) == 1 and \
            all([1 <= d and d <= 3 for d in [abs(p1-p2) for p1, p2 in pairs]])
    else:
        for s in range(len(line)):
            new_line = line[:s] + line[s+1:]
            if check_safe(new_line):
                return True
        return False
    
def part1():
    ans = 0
    for line in lines:
        if check_safe(line):
            ans += 1
    return ans

def part2():
    ans = 0
    for line in lines:
        if check_safe(line, True):
            ans += 1
    return ans

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
