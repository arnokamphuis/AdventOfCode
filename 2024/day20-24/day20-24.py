
# read command-line parameters and based on that read the input file
import sys
from typing import Counter
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day20-{runtype}.txt", "r")

lines = [list(line.strip()) for line in text_file.readlines()]

map = set()
for y, line in enumerate(lines):
    for x, char in enumerate(line):
        if char != "#":
            map.add((x, y))
        if char == "S":
            start = (x, y)
        if char == "E":
            end = (x, y)
            
def get_neighbors(pos):
    x, y = pos
    return [p for p in [(x+1, y), (x-1, y), (x, y+1), (x, y-1)] if p in map]

def get_path(start, end):
    current = start
    path = [current]
    while current != end:
        for next  in [p for p in get_neighbors(current) if p not in path]:
            path.append(next)
            break
        current = next
    return path

def check_walls(p1, p2):
    for x in range(min(p1[0], p2[0]), max(p1[0], p2[0])+1):
        for y in range(min(p1[1], p2[1]), max(p1[1], p2[1])+1):
            if (x, y) not in map:
                return True
    return False

def get_all_cheats(path, max_cheats):
    cheats = []
    for p_index, p in enumerate(path):
        for pos_index in range(p_index+1, len(path)):
            pos = path[pos_index]
            distance = abs(pos[0] - p[0]) + abs(pos[1] - p[1])
            if distance <= 1:
                continue
            
            saving = pos_index - p_index - distance
            if distance <= max_cheats and check_walls(p, pos) and saving > 0:
                cheats.append(saving)
    return cheats
        
def savings(part):
    max_chats = 2 if part == 1 else 20
    savings_counter = Counter(get_all_cheats(get_path(start, end), max_chats))
    return sum([sc[1] for sc in savings_counter.items() if sc[0] >= 100])

def part1():
    return savings(1)

def part2():
    return savings(2)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
