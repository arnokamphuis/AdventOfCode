
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

def find_antinodes(a, b):
    dr = a[0] - b[0]
    dc = a[1] - b[1]
    
    antinodes = []
    antinode = (a[0] + dr, a[1] + dc)
    if in_bounds(antinode) and antinode != b:
        antinodes.append(antinode)
    antinode = (b[0] - dr, b[1] - dc)
    if in_bounds(antinode) and antinode != a:
        antinodes.append(antinode)
    return antinodes

def find_antinodes_harmonics(a, b):
    dr = a[0] - b[0]
    dc = a[1] - b[1]
    
    antinodes = []
    l = 1
    antinode = (a[0] + l*dr, a[1] + l*dc)
    while in_bounds(antinode):
        antinodes.append(antinode)
        l += 1
        antinode = (a[0] + l*dr, a[1] + l*dc)
    l = 1
    antinode = (b[0] - l*dr, b[1] - l*dc)
    while in_bounds(antinode):
        antinodes.append(antinode)
        l += 1
        antinode = (b[0] - l*dr, b[1] - l*dc)
    return [*antinodes, a, b]

def part1():
    all_antinodes = set()
    for freq in map:
        antenna = map[freq]
        for a in antenna:
            for b in antenna:
                if a == b: continue
                antinodes = find_antinodes(a, b)
                all_antinodes.update(antinodes)
    return len(all_antinodes)

def part2():
    all_antinodes = set()
    for freq in map:
        antenna = map[freq]
        for a in antenna:
            for b in antenna:
                if a == b: continue
                antinodes = find_antinodes_harmonics(a, b)
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
    
