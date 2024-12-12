
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

text_file = open(f"day12-{runtype}.txt", "r")

lines = [[ch for ch in line.strip()] for line in text_file.readlines()]

plant_types = set()
map = defaultdict(set)
for r, line in enumerate(lines):
    for c, ch in enumerate(line):
        map[ch].add((c, r))
        plant_types.add(ch)

def remove_connected(map, sc, sr):
    Q = [(sc, sr)]
    to_remove = set()
    while len(Q) > 0:
        c, r = Q.pop()
        if (c, r) not in map or (c, r) in to_remove:
            continue
        to_remove.add((c, r))
        for dc, dr in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            if (c+dc, r+dr) in map:
                Q.append((c+dc, r+dr))
    return to_remove

def find_connected(map):
    results = {}
    for plant in plant_types:
        results[plant] = []
        remaining = deepcopy(map[plant])
        for c, r in remaining:
            cc = remove_connected(remaining, c, r)
            remaining = remaining.difference(cc)
            if len(cc) > 0:
                results[plant].append(cc)
    return results

def part1():
    results = find_connected(map)

    res1 = 0
    for plant in plant_types:
        for res in results[plant]:
            area = len(res)
            perimeter = 0
            for c, r in res:
                perimeter += 4 - len([(c+dc, r+dr) for dc, dr in [(0, 1), (0, -1), (1, 0), (-1, 0)] if (c+dc, r+dr) in res])
            res1 += area * perimeter

    return res1

def part2():
    res2 = 0
    results = find_connected(map)
    
    for plant in plant_types:
        for res in results[plant]:
            perimeter = {}
            # first find all points on the perimeter in a set for all 4 directions
            for (c,r) in res:
                for (dc, dr) in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
                    if (c+dc, r+dr) not in res:
                        if (dc, dr) not in perimeter:
                            perimeter[(dc, dr)] = set()
                        perimeter[(dc, dr)].add((c, r))
            sides = 0
            # per direction find all the connected components of the perimeter
            # every connected component is a side
            for dir, perimeter_set in perimeter.items():
                seen_perimeter = set()
                for (c_pr, r_pr) in perimeter_set:
                    if (c_pr, r_pr) not in seen_perimeter:
                        # new start of a connected component found
                        sides += 1
                        Q = [(c_pr, r_pr)]
                        while len(Q)>0:
                            (c, r) = Q.pop()
                            if (c, r) not in seen_perimeter:
                                seen_perimeter.add((c, r))
                                Q = [*Q, *[(c+dc, r+dr) for (dc, dr) in [(dir[1], -dir[0]), (-dir[1], dir[0])] if (c+dc, r+dr) in perimeter_set]]
            res2 += sides * len(res)
    return res2

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
