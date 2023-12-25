
# read command-line parameters and based on that read the input file
from collections import deque
from math import floor
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
            
def can_reach(map, sr, sc, steps):
    reachable = set()
    visited = set()
    visited.add((sr, sc))

    q = deque([(sr, sc, steps)])
    while q:
        cr, cc, s = q.popleft()

        if s%2 == 0:
            reachable.add((cr, cc))
        if s == 0:
            continue

        dirs = [(0,1), (0,-1), (1,0), (-1,0)]
        for d in dirs:
            r = cr+d[0]
            c = cc+d[1]
            if r < 0 or r >= len(map) or c < 0 or c >= len(map[r]) or map[r][c] == "#" or (r, c) in visited:
                continue
            visited.add((r, c))
            q.append((r, c, s-1))
    return len(reachable)

def part1():
    sr, sc = find_s(map)
    return can_reach(map, sr, sc, 64)

def part2():
    if runtype == "test":
        return 0

    sr, sc = find_s(map)

    size = len(map)
    steps = 26501365

    grid_width = steps//size - 1
    odd_grids = (grid_width // 2 * 2 + 1) ** 2
    even_grids = ((grid_width+1) // 2 * 2) ** 2

    odd_points = can_reach(map, sr, sc, size * 2 + 1)
    even_points = can_reach(map, sr, sc, size * 2)

    top_corner = can_reach(map, size - 1, sc, size - 1)
    bottom_corner = can_reach(map, 0, sc, size - 1)
    left_corner = can_reach(map, sr, size-1, size - 1)
    right_corner = can_reach(map, sr, 0, size - 1)

    small_top_right = can_reach(map, size-1, 0, size // 2 - 1)
    small_bottom_right = can_reach(map, 0, 0, size // 2 - 1)
    small_bottom_left = can_reach(map, 0, size-1, size // 2 - 1)
    small_top_left = can_reach(map, size-1, size-1, size // 2 - 1)

    large_top_right = can_reach(map, size-1, 0, (size * 3) // 2 - 1)
    large_bottom_right = can_reach(map, 0, 0, (size * 3) // 2 - 1)
    large_bottom_left = can_reach(map, 0, size-1, (size * 3) // 2 - 1)
    large_top_left = can_reach(map, size-1, size-1, (size * 3) // 2 - 1)

    res = odd_grids * odd_points + even_grids * even_points + \
        top_corner + bottom_corner + left_corner + right_corner + \
        (grid_width + 1) * (small_top_right + small_bottom_right + small_bottom_left + small_top_left) + \
        grid_width * (large_top_right + large_bottom_right + large_bottom_left + large_top_left)
    
    return res

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
