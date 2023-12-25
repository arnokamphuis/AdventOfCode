import sys
from collections import deque, defaultdict 
from math import atan2

runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day10-{}.txt".format(runtype), "r")

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

expand = {
    '-': [( 0,-1), (0,0), ( 0, 1)],
    '|': [(-1, 0), (0,0), ( 1, 0)],
    '7': [( 0,-1), (0,0), ( 1, 0)],
    'L': [(-1, 0), (0,0), ( 0, 1)],
    'F': [( 1, 0), (0,0), ( 0, 1)],
    'J': [( 0,-1), (0,0), (-1, 0)]
}

contract = {
    ('-',0): [( 1, 0)], ('-',2): [(-1, 0)], 
    ('|',1): [( 0,-1)], ('|',3): [( 0, 1)],
    ('7',3): [( 0, 1), (-1, 1), (-1, 0)], ('7',0): [( 1,-1)],
    ('L',1): [( 0,-1), ( 1,-1), ( 1, 0)], ('L',2): [(-1, 1)],
    ('F',2): [(-1, 0), (-1,-1), ( 0,-1)], ('F',3): [( 1, 1)],
    ('J',0): [( 1, 0), ( 1, 1), ( 0, 1)], ('J',1): [(-1,-1)]
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

distances = defaultdict(int)
previous = defaultdict(lambda: (None,None))

def part1():
    global distances, previous
    return len(find_path())//2

def find_path():
    angle = 0
    mid = (R//2, C//2)
    dir_index = {(0,-1): 2, (0,1): 0, (1,0): 1, (-1,0): 3}
    dir_act   = [(0,1), (1,0), (0,-1), (-1,0)]
    r,c = find_s()
    pipe = s_pipe()
    n_dir = next_dir[pipe][1]
    path = []
    r += n_dir[0]
    c += n_dir[1]
    path.append((r,c))
    while lines[r][c] != 'S':
        pipe = lines[r][c]
        n_dir = dir_act[list(filter(lambda nd: nd != (dir_index[n_dir]+2)%4 , optional_dir[pipe]))[0]]
        r += n_dir[0]
        c += n_dir[1]
        if (r,c) != mid:
            angle += atan2((r-mid[0]),(c-mid[1]))
        path.append((r,c))
    if angle > 0:
        path.reverse()
    return path

def find_contour(path):
    contour = set()
    dir_index = {(0,-1): 2, (0,1): 0, (1,0): 1, (-1,0): 3}
    for i in range(len(path)):
        cr, cc = path[i-1]
        r, c = path[i]
        dr, dc = r-cr, c-cc
        contour.add(((r,c), dir_index[(dr,dc)]))
    return contour

def find_contraction(path, contour):
    global lines
    contraction = defaultdict(set)
    for (r,c), d in contour:
        ctr = contract[(lines[r][c],d)]
        for e in ctr:
            newr = r+e[0]
            newc = c+e[1]
            if newr >= 0 and newr < R and newc >= 0 and newc < C:
                if (newr, newc) not in path:
                    contraction[(newr, newc)].add(e)
    return contraction


def part2():
    global distances, previous

    path = find_path()
    contour = find_contour(path)

    r,c = find_s()
    lines[r][c] = s_pipe()

    contourmap = defaultdict(int)
    for (r,c), d in contour:
        contourmap[(r,c)] = d

    contraction = find_contraction(path, contour)

    inset = set()
    candidates = set(list(contraction.keys()))
    while candidates:
        r,c = candidates.pop()
        q = deque([(r,c)])
        component = set()
        component.add((r,c))
        while q:
            r, c = q.popleft()
            for dr, dc in ((0, 1), (0, -1), (1, 0), (-1, 0)):
                newr, newc = r + dr, c + dc
                assert 0 <= newr < R and 0 <= newc < C
                if (newr, newc) not in path and (newr, newc) not in component:
                    component.add((newr, newc))
                    q.append((newr, newc))

        inset = inset.union(component)
        candidates = candidates.difference(component)
    return len(inset)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
