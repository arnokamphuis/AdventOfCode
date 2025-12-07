
# read command-line parameters and based on that read the input file
from collections import defaultdict
from copy import deepcopy
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day07-{runtype}.txt", "r")

lines = [list(line.strip()) for line in text_file.readlines()]

beam_origin = None
splitters = defaultdict(list)
for y in range(len(lines)):
    for x in range(len(lines[y])):
        if lines[y][x] == "S":
            beam_origin = (x, y)
        if lines[y][x] == "^":
            splitters[y].append(x)

height = len(lines)
width = len(lines[0])


def count():
    split_counter = 0
    pos = defaultdict(int)
    pos[beam_origin] = 1

    while True:
        future_splitters = {
            y: xs
            for (y, xs) in splitters.items()
            if any([py<y and px in xs for (px, py) in pos.keys()])
        }
        if not future_splitters:
            break

        min_key = min(future_splitters.keys())

        new_positions = defaultdict(int)
        for (x,y) in deepcopy(pos):
            for fx in future_splitters[min_key]:
                if x == fx:
                    split_counter += 1
                    new_positions[(x-1, min_key)] += pos[(x,y)]
                    new_positions[(x+1, min_key)] += pos[(x,y)]
                    del pos[(x,y)]

        for p in pos:
            new_positions[(p[0], min_key)] += pos[p]
        pos = new_positions

    return split_counter, sum(pos.values())

def part1():
    return count()[0]

def part2():
    return count()[1]

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
