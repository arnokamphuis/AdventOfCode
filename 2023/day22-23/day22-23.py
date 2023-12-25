
# read command-line parameters and based on that read the input file
from collections import defaultdict
from copy import deepcopy
from functools import reduce
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day22-{runtype}.txt", "r")

lines = [line.split("~") for line in text_file.readlines()]
lines = [[tuple(line[0].strip().split(',')), tuple(line[1].strip().split(','))] for line in lines]

bricks = [[
    [int(c) for c in line[0]],
    [int(c) for c in line[1]]] for line in lines]

size = [
    [min([min(brick[0][i], brick[1][i]) for brick in bricks]), 
     max([max(brick[0][i], brick[1][i]) for brick in bricks])]
     for i in range(3)]

grid = defaultdict()

def can_fall(brick):
    global grid
    z = min(brick[0][2], brick[1][2])-1
    free = all( (x,y,z) not in grid for x in range(brick[0][0], brick[1][0]+1) for y in range(brick[0][1], brick[1][1]+1))
    return z > 0 and free

def add_to_grid(index, brick):
    global grid
    for z in range(brick[0][2], brick[1][2]+1):
        for x in range(brick[0][0], brick[1][0]+1):
            for y in range(brick[0][1], brick[1][1]+1):
                assert ((x,y,z) not in grid), f"({x},{y},{z}) already in grid {brick}"
                grid[(x,y,z)] = index

def remove_from_grid(brick):
    global grid
    for z in range(brick[0][2], brick[1][2]+1):
        for x in range(brick[0][0], brick[1][0]+1):
            for y in range(brick[0][1], brick[1][1]+1):
                del grid[(x,y,z)]  

def can_be_removed(index, brick):
    global grid
    res = False
    remove_from_grid(brick)
    if not any([can_fall(b) for b in bricks if b!=brick]):
        res = True
    add_to_grid(index, brick)
    return res

def part1():
    global grid

    sorted_bricks = sorted(enumerate(bricks), key=lambda a: min(a[1][0][2], a[1][1][2]))
    for index, sb in sorted_bricks:
        while can_fall(sb):
            bricks[index][0][2] -= 1
            bricks[index][1][2] -= 1
        add_to_grid(index, sb)

    candidate = 0
    for index, sb in sorted_bricks:
        candidate += 1 if can_be_removed(index, sb) else 0
    return candidate

def get_support(brick):
    global grid
    z = min(brick[0][2], brick[1][2])-1
    support = [grid[(x,y,z)] if (x,y,z) in grid else None for x in range(brick[0][0], brick[1][0]+1) for y in range(brick[0][1], brick[1][1]+1)]
    if len(support) == 0 or all(s is None for s in support):
        return set([-1])
    return set([s for s in support if s is not None])

def count_falling(support, falling):
    new_falling = deepcopy(falling)
    supported_bricks_by_falling = [bi for bi, sup in support.items() if all(s in falling for s in sup)]
    for sbf in supported_bricks_by_falling:
        if len(support[sbf].difference(falling)) == 0:
            new_falling.add(sbf)
    if new_falling == falling:
        return len(falling)
    else:
        return count_falling(support, new_falling)

def part2():
    res = 0
    support = {index: get_support(brick) for index, brick in enumerate(bricks)}
    for bi in range(len(bricks)):
        res += count_falling(support, set([bi]))-1
    return res

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
