
# read command-line parameters and based on that read the input file
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
    box = [[] for _ in range(256)]
    for step in lines:
        label, id = re.split('-|=', step)
        h = hash(label)
        ri = [i for i, x in enumerate(box[h]) if x[0] == label]
        if '-' in step: # remove lens from box
            if len(ri) > 0:
                del box[h][ri[0]]
        else: # add lens to box
            if len(ri) > 0:
                box[h][ri[0]] = (label,int(id))
            else:
                box[h].append((label,int(id)))

    filled_boxes =  [(i, b) for i, b in enumerate(box,1) if len(b) > 0]

    return sum([fbn * sum([ sid*slot[1] for sid, slot in enumerate(b,1) ]) for fbn, b in filled_boxes])

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
