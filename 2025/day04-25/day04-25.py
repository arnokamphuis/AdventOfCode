
# read command-line parameters and based on that read the input file
from copy import deepcopy
import sys
from time import sleep
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day04-{runtype}.txt", "r")

data = [list(line.strip()) for line in text_file.readlines()]
rolls = { (x,y) for y, row in enumerate(data) for x, val in enumerate(row) if val == '@' }

def remove(rolls_in):
    to_be_removed = set()
    nbrs = [(1,0), (0,1), (1,1), (1,-1), (-1,-1), (-1,1), (0,-1), (-1,0)]
    for (x,y) in rolls_in:
        neighbor_values = [d for d in nbrs if (x+d[0],y+d[1]) in rolls_in ]
        if len(neighbor_values) < 4:
            to_be_removed.add((x,y))
    return to_be_removed


def part1():
    return len(remove(rolls))

def part2():
    wall = deepcopy(rolls)
    all_removed = set()
    while True:
        to_be_removed = remove(wall)
        all_removed |= to_be_removed
        if len(to_be_removed) == 0:
            break
        wall -= to_be_removed
    return len(all_removed)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
