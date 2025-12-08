
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

text_file = open(f"day08-{runtype}.txt", "r")

junctions = [tuple(map(int, line.strip().split(','))) for line in text_file.readlines()]

def all_distances(junctions):
    dist_map = defaultdict(list)
    for b1 in junctions:
        for b2 in junctions:
            if b1 != b2:
                dist = (b1[0]-b2[0])**2 + (b1[1]-b2[1])**2 + (b1[2]-b2[2])**2
                dist_map[dist].append((min(b1, b2), max(b1, b2)))
    return {k: dist_map[k] for k in sorted(dist_map.keys())}

def part1():
    n = 1000 if runtype == 'real' else 10
    distances = list(all_distances(junctions).items())[:n]
    directly_connected = set()

    connected_components = [set([j]) for j in junctions]

    for d, pair in distances:
        b1, b2 = pair[0]

        cc1 = [cc for cc in connected_components if b1 in cc][0]
        cc2 = [cc for cc in connected_components if b2 in cc][0]

        if cc1 != cc2:
            cc1.update(cc2)
            connected_components.remove(cc2)
            directly_connected |= {(min(b1, b2), max(b1, b2))}
    cclen = sorted([len(cc) for cc in connected_components], reverse=True)

    return cclen[0] * cclen[1] * cclen[2]

def part2():
    distances = list(all_distances(junctions).items())
    directly_connected = set()

    connected_components = [set([j]) for j in junctions]

    for d, pair in distances:
        b1, b2 = pair[0]

        cc1 = [cc for cc in connected_components if b1 in cc][0]
        cc2 = [cc for cc in connected_components if b2 in cc][0]

        if cc1 != cc2:
            cc1.update(cc2)
            connected_components.remove(cc2)
            directly_connected |= {(min(b1, b2), max(b1, b2))}

        if len(connected_components) == 1:
            return b1[0] * b2[0]
    return 0

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
