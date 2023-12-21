
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day21-{runtype}.txt", "r")

map = [[c for c in line.strip()] for line in text_file.readlines()]

def find_s(map):
    for r in range(len(map)):
        for c in range(len(map[r])):
            if map[r][c] == "S":
                return r, c
            
DP = {}
def can_reach(map, sr, sc, steps):
    if (sr, sc, steps) in DP:
        return DP[(sr, sc, steps)]
    
    reachable = set([])
    if steps == 0:
        reachable.add((sr, sc))
    else:
        dirs = [(0,1), (0,-1), (1,0), (-1,0)]
        for d in dirs:
            r = sr+d[0]
            c = sc+d[1]
            if r < 0 or r >= len(map) or c < 0 or c >= len(map[r]):
                continue
            if map[r][c] == "#":
                continue
            next_reachable = can_reach(map, r, c, steps-1)
            reachable = reachable.union(next_reachable)
    DP[(sr, sc, steps)] = reachable
    return reachable

def part1():
    sr, sc = find_s(map)
    reachable = can_reach(map, sr, sc, 64)
    return len(reachable)

def part2():
    return 0

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
