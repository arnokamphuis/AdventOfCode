
# read command-line parameters and based on that read the input file
from collections import defaultdict
from functools import reduce
import re
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day15-{}.txt".format(runtype), "r")

lines = [list(map(lambda x: x.strip(), 
                  [line.split(',') for line in text_file.readlines()][0]))][0]

def hash(current_str):
    return reduce(lambda x,y: (x + ord(y)) * 17 % 256, current_str, 0)

def part1():
    return reduce(lambda x, y: x + y, [hash(line) for line in lines], 0)

def part2():
    focal_lengths = {}
    box = defaultdict(list)
    for step in lines:
        label, id = re.split('-|=', step)
        h = hash(label) + 1
        if '-' in step: # remove lens from box
            if h in box and label in box[h]:
                if len(box[h]) <= 1:
                    del box[h]
                else:
                    box[h].remove(label)
        else: # add lens to box
            if label not in box[h]:
                box[h].append(label)
            focal_lengths[label] = int(id)

    return sum([fbn * sum([ sid*focal_lengths[label] for sid, label in enumerate(b,1) ]) for fbn, b in box.items()])

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
