
# read command-line parameters and based on that read the input file
from collections import defaultdict
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day10-{runtype}.txt", "r")

chart = [list(line.strip()) for line in text_file.readlines()]

R, C = len(chart), len(chart[0])
map = { (c, r): int(chart[r][c]) for r in range(R) for c in range(C) if chart[r][c] != '.' }

def print_map(m):
    print('-'*40)
    for r in range(R):
        for c in range(C):
            if (c, r) in m:
                print(m[(c,r)], end="")
            else:
                print(".", end="")
        print()
    print('-'*40)
    
def print_fill(m, f):
    for r in range(R):
        for c in range(C):
            if (c, r) in f:
                print(m[(c,r)], end="")
            else:
                print(".", end="")
        print()

def in_bounds(c, r):
    return c >= 0 and c < C and r >= 0 and r < R

def flood_fill(m, f, c, r, val):
    if (c, r) in f and val in f[(c, r)]:
        return
    
    f[(c, r)].append(val)
    
    for dc, dr in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        next = (c+dc, r+dr)
        if next in m and m[next]-m[(c, r)] == 1:
            flood_fill(m, f, c+dc, r+dr, val)

def flood_fill_2(m, f, c, r, val):
    f[(c, r)][val] += 1
    
    for dc, dr in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        next = (c+dc, r+dr)
        if next in m and m[next]-m[(c, r)] == 1:
            flood_fill_2(m, f, c+dc, r+dr, val)

def part1():
    fill = defaultdict(list)
    trailheads = [ p for p in map if map[p] == 0]
    nines = [ p for p in map if map[p] == 9]

    for i, th in enumerate(trailheads):
        flood_fill(map, fill, th[0], th[1], i)

    return sum([len(fill[(c, r)]) for c, r in nines])

def part2():
    fill = { (c,r): defaultdict(int) for r in range(R) for c in range(C) }
    trailheads = [ p for p in map if map[p] == 0]
    nines = [ p for p in map if map[p] == 9]
    for i, th in enumerate(trailheads):
        flood_fill_2(map, fill, th[0], th[1], i)      
    trail_counter = defaultdict(int)
    for (c,r) in nines:
        for i, c in fill[(c,r)].items():
            trail_counter[i] += c
    return sum(trail_counter.values())

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
