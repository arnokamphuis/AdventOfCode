
# read command-line parameters and based on that read the input file
from collections import defaultdict
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day16-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]
R, C = len(lines), len(lines[0])
map = set()

for r in range(R):
    for c in range(C):
        if lines[r][c] != "#":
            map.add((c, r))
        if lines[r][c] == "S":
            start = (c, r)
        elif lines[r][c] == "E":
            end = (c, r)
start_dir = (1,0)

dirs = [(1,0), (0,1), (-1,0), (0,-1)]

def neighbors(pos, dir):
    nbs = []
    index = dirs.index(dir)
    for dd in [-1, 1]:
        new_dir = dirs[(index + dd) % 4]
        nbs.append(((pos, new_dir), 1000))
    new_pos = (pos[0] + dir[0], pos[1] + dir[1])
    if new_pos in map:
        nbs.append(((new_pos, dir),1))
    return nbs

def search(map, start, start_dir, end, part):
    lowest_cost = defaultdict(lambda: float("inf"))
    lowest_cost[(start, start_dir)] = 0
    backtrack = defaultdict(lambda: [])
    best_cost = float("inf")
    end_states = set()

    queue = [(0, start, start_dir)]
    while queue:
        c, pos, dir = queue.pop(0)
        if c <= lowest_cost[(pos, dir)]:
            if pos == end:
                if c > best_cost:
                    continue
                best_cost = c
                end_states.add((pos, dir))
                
            for ((p, d), ec) in neighbors(pos, dir):
                if p in map:
                    lowest = lowest_cost[(p, d)]
                    if c + ec > lowest:
                        continue
                    if c + ec < lowest:
                        backtrack[(p, d)] = set()
                        lowest_cost[(p, d)] = c + ec
                    backtrack[(p, d)].add((pos, dir))
                    queue.append((c + ec, p, d))

    if part == 1:
        return best_cost
            
    states = list(end_states)
    visited_positions = set(end_states)
    while states:
        (p,d) = states.pop()
        for (np, nd) in backtrack[(p,d)]:
            if (np, nd) in visited_positions:
                continue
            visited_positions.add((np, nd))
            states.append((np, nd))
    return len(set([pos for (pos, _) in visited_positions]))
    

def part1():
    return search(map, start, start_dir, end, 1)

def part2():
    return search(map, start, start_dir, end, 2)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
