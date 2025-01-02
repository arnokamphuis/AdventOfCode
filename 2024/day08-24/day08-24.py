
# read command-line parameters and based on that read the input file
from collections import defaultdict
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day08-{runtype}.txt", "r")

lines = [list(line.strip()) for line in text_file.readlines()]
R, C = len(lines), len(lines[0])

map = {}
for r in range(R):
    for c in range(C):
        if lines[r][c] == ".": continue
        freq = ord(lines[r][c])
        if freq not in map:
            map[freq] = []
        map[freq].append((r, c))

def in_bounds(pos):
    r, c = pos
    return r >= 0 and r < R and c >= 0 and c < C

def find_antinodes_harmonics(a, b, harmonics=False):
    dr = a[0] - b[0]
    dc = a[1] - b[1]
    
    antinodes = []
    l = 1
    antinode = (a[0] + l*dr, a[1] + l*dc)
    while in_bounds(antinode):
        if not harmonics and antinode == b: break

        antinodes.append(antinode)

        if not harmonics: break

        l += 1
        antinode = (a[0] + l*dr, a[1] + l*dc)

    if not harmonics: return antinodes        
    return [*antinodes, a]

def part1():
    all_antinodes = set()
    for freq in map:
        antenna = map[freq]
        for a in antenna:
            for b in antenna:
                if a == b: continue
                antinodes = [*find_antinodes_harmonics(a, b), 
                             *find_antinodes_harmonics(b, a)]
                all_antinodes.update(antinodes)
    return len(all_antinodes)

def part2():
    all_antinodes = set()
    for freq in map:
        antenna = map[freq]
        for a in antenna:
            for b in antenna:
                if a == b: continue
                antinodes = [*find_antinodes_harmonics(a, b, True), 
                             *find_antinodes_harmonics(b, a, True)]
                all_antinodes.update(antinodes)
    return len(all_antinodes)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
