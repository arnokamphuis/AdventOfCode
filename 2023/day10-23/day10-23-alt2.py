import sys
from collections import deque, defaultdict 
from math import atan2

runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day10-{runtype}.txt", "r")

lines = [[c for c in line.strip()] for line in text_file.readlines()]
R = len(lines)
C = len(lines[0])

def find_s():
    for r in range(R):
        for c in range(C):
            if lines[r][c] == 'S':
                return r, c

allowed = {
    (0,1):  ['-', 'J', '7'], # right
    (0,-1): ['-', 'F', 'L'], # left
    (1,0):  ['|', 'J', 'L'], # down
    (-1,0): ['|', 'F', '7']  # up
}

next_dir = {
    '-': [( 0,-1), ( 0, 1)],
    '|': [( 1, 0), (-1, 0)],
    '7': [( 0,-1), ( 1, 0)],
    'L': [( 0, 1), (-1, 0)],    
    'F': [( 0, 1), ( 1, 0)],
    'J': [( 0,-1), (-1, 0)]
}

optional_dir = {
    '-': [0,2],
    '|': [1,3],
    '7': [2,1],
    'L': [3,0],
    'F': [0,1],
    'J': [2,3]
}

def s_pipe():
    start = find_s()
    connected = []
    for dr, dc in ((0, 1), (0, -1), (1, 0), (-1, 0)):
        r, c = start
        newr, newc = r + dr, c + dc
        if 0 <= newr < R and 0 <= newc < C:
            if lines[newr][newc] in allowed[(dr, dc)]:
                connected.append((dr, dc))

    possible_s = [
        [[(0, -1), (1, 0)], '7'],
        [[(0, 1), (1, 0)], 'F'],
        [[(0, -1), (-1, 0)], 'J'],
        [[(0, 1), (-1, 0)], 'L']
    ]

    for pos, dir in possible_s:
        if connected == pos:
            return dir

def determinte_length(points):
    return sum( \
        [abs(points[i-1][0] - points[i][0]) + abs(points[i-1][1] - points[i][1]) \
         for i in range(len(points))])

def calculate_interior_area(points):
    boundary_points_count = determinte_length(points)
    
    # first use shoelace formula to calculate area of polygon
    # https://en.wikipedia.org/wiki/Shoelace_formula
    area = abs(sum( points[i][0] * (points[i-1][1] - points[(i+1) % len(points)][1]) for i in range(len(points)))) // 2

    # then use Pick's theorem to calculate area of polygon with integer coordinates
    # https://en.wikipedia.org/wiki/Pick%27s_theorem
    interior_points_count = area - boundary_points_count//2 + 1

    return interior_points_count

def find_path():
    r,c = find_s()
    pipe = s_pipe()
    points = [(r,c)]

    dir_index = {(0,-1): 2, (0,1): 0, (1,0): 1, (-1,0): 3}
    dir_act   = [(0,1), (1,0), (0,-1), (-1,0)]

    n_dir = next_dir[pipe][0]

    (r,c) = (r+n_dir[0], c+n_dir[1])
    while lines[r][c] != 'S':
        pipe = lines[r][c]
        n_dir = dir_act[list(filter(lambda nd: nd != (dir_index[n_dir]+2)%4 , optional_dir[pipe]))[0]]
        r += n_dir[0]
        c += n_dir[1]
        if lines[r][c] != '.':
            points.append((r,c))
    return points

def part1():
    points = find_path()
    return determinte_length(points) // 2


def part2():
    points = find_path()
    return calculate_interior_area(points)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
