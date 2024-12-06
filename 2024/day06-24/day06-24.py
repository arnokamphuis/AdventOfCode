
# read command-line parameters and based on that read the input file
import sys
from typing import Counter
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day06-{runtype}.txt", "r")

lines = [[ch for ch in line.strip()] for line in text_file.readlines()]
guard_pos = None 
guard_dir = None
start = None

dirs = { "^": 3, "v": 1, "<": 2, ">": 0 }
R, C = len(lines), len(lines[0])

obstacles = []
for i, line in enumerate(lines):
    for j, char in enumerate(line):
        if char == "#":
            obstacles.append((j, i))
        if char not in ['#', '.']:
            guard_pos = (j, i)
            guard_dir = dirs[char]

start = guard_pos
start_dir = guard_dir

def in_range(pos):
    return 0 <= pos[0] < C and 0 <= pos[1] < R

pos_dirs = {0: (1, 0), 1: (0, 1), 2: (-1, 0), 3: (0, -1)}

def find_turn_points():
    global guard_pos, guard_dir
    guard_pos = start
    guard_dir = start_dir
    turn_points = []
    visited = set()
    while True:
        newpos = (guard_pos[0] + pos_dirs[guard_dir][0], guard_pos[1] + pos_dirs[guard_dir][1])
        if newpos in obstacles:
            turn_points.append((guard_pos, pos_dirs[guard_dir]))
            guard_dir = (guard_dir + 1) % 4
        else:
            if not in_range(newpos):
                break
            guard_pos = newpos
            visited.add(guard_pos)
    return turn_points, visited


def part1():
    _, visited = find_turn_points()
    return len(visited)

def part2():
    res2 = 0
    for o_r in range(R):
        for o_c in range(C):
            guard_pos = start
            guard_dir = start_dir
            visited = set()
            visited_including_dir = set()
            while True:
                if (guard_pos + (guard_dir,)) in visited_including_dir:
                    res2 += 1
                    break
                visited_including_dir.add(guard_pos + (guard_dir,))
                visited.add(guard_pos)
                newpos = (guard_pos[0] + pos_dirs[guard_dir][0], guard_pos[1] + pos_dirs[guard_dir][1])
                if not in_range(newpos):
                    break
                if newpos in obstacles or newpos == (o_c, o_r):
                    guard_dir = (guard_dir + 1) % 4
                else:
                    guard_pos = newpos
    return res2

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
