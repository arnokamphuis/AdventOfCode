
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

def intersect(fb, fe, tb, te):
    assert fb[0] <= fe[0]
    assert fb[1] <= fe[1]
    assert tb[0] <= te[0]
    assert tb[1] <= te[1]

    # print(f"{fb} {fe} {tb} {te}")

    x1 = fb[0]
    y1 = fb[1]

    x2 = fe[0]
    y2 = fe[1]

    x3 = tb[0]
    y3 = tb[1]

    x4 = te[0]
    y4 = te[1]

    x5 = max(x1, x3)
    y5 = max(y1, y3)
    x6 = min(x2, x4)
    y6 = min(y2, y4)
 
    # print(not (x5 > x6 or y5 > y6))
    # no intersection
    return not (x5 > x6 or y5 > y6)

def part1():
    global grid

    sorted_bricks = sorted(enumerate(bricks), key=lambda a: min(a[1][0][2], a[1][1][2]))
    drop_amount = defaultdict(int)
    drop_amount[0] = 0
    support = {}
    all_above = {}
    for f, (bi, brick) in enumerate(sorted_bricks):
        above = {}
        for t in range(f+1, len(sorted_bricks)):
            si, sup = sorted_bricks[t]
            assert bi != si

            bricks_intersect = intersect(brick[0], brick[1], sup[0], sup[1])
            if bricks_intersect:

                assert brick[0][2] <= brick[1][2]
                min_sh = min(sup[0][2], sup[1][2])
                max_bh = max(brick[0][2], brick[1][2]) - drop_amount[bi]
                assert max_bh <= min_sh, f"{brick} {sup} {max_bh} {min_sh} {drop_amount[bi]}"

                if max_bh < min_sh:
                    drop_amount[si] = min_sh - max_bh - 1
                    above[si] = min_sh - max_bh - 1
        all_above[bi] = deepcopy(above)
        # print(f"Above {bi}: {above}")
        
    for bi, brick in enumerate(bricks):
        drops = [(aai, aa[bi]) for aai, aa in all_above.items() if bi in aa.keys()]
        if len(drops) == 0:
            continue
        minimum_drop = min(drops, key=lambda a: a[1])
        support[bi] = set([d[0] for d in drops if d[1] == minimum_drop[1]])

    cannot_be_disintegrated = set([bi for bi in range(len(bricks)) if len([si for si, sup in support.items() if len(sup.difference(set([bi]))) == 0]) > 0])
    return len(bricks) - len(cannot_be_disintegrated)

    for i in range(len(bricks)):
        print(f"supported by {i}: {supported[i]}")

    for i, v in enumerate(supported):
        print(f"{i}: {v}  -> {[support[sup].difference(set([i])) for sup in v]}")
    # print(support)
    return 0
    res = 0
    for bi, brick in enumerate(bricks):
        print(f"{chr(65+bi)}")
        # print(f"{bi}: {brick} {support[bi]} {supported[bi]}")
        supported_bricks_by_bi = supported[bi]

        temp = [support[sbybi].difference(set([bi])) for sbybi in supported_bricks_by_bi]
        print("    ",temp)
        if any(len(support[sbybi].difference(set([bi]))) == 0 for sbybi in supported_bricks_by_bi):
            print("can be removed")
            res += 1
        else:
            print("can not be removed")

    print(support)
    print(supported)
    return res

    # sorted_bricks = sorted(enumerate(bricks), key=lambda a: min(a[1][0][2], a[1][1][2]))
    # for index, sb in sorted_bricks:
    #     while can_fall(sb):
    #         bricks[index][0][2] -= 1
    #         bricks[index][1][2] -= 1
    #     add_to_grid(index, sb)

    # candidate = 0
    # for index, sb in sorted_bricks:
    #     candidate += 1 if can_be_removed(index, sb) else 0
    # return candidate

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
    
