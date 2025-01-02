
# read command-line parameters and based on that read the input file
from collections import defaultdict
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day19-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]
patterns = lines[0].split(", ")
designs = lines[2:]

DP = defaultdict(int)
def count(design, target):
    global DP
    key = design
    if key in DP:
        return DP[key]
    
    if len(design) > len(target):
        return 0
    
    if design == target:
        return 1
    
    res = 0
    
    for pattern in patterns:
        new_design = design + pattern
        if new_design == target[:len(new_design)]:
            res += count(new_design, target)
    
    DP[key] = res
    return res

def count_all(part):
    global DP
    ans = 0
    for design in designs:
        DP = defaultdict(int)
        res = count("", design)
        ans += res if part == 2 else 1 if res > 0 else 0
    return ans

def part1():
    return count_all(1)

def part2():
    return count_all(2)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
