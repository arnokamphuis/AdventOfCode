import sys
from collections import deque, defaultdict 

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

def part1():
    global distances
    start = find_s()
    q = deque([start])
    distances[start] = 0
    previous = defaultdict(lambda: (None,None))
    while q:
        current = q.popleft()
        r, c = current
        pipe = lines[r][c]
        if pipe == 'S':
            pipe = s_pipe()
        for dr, dc in next_dir[pipe]:
            newr, newc = r + dr, c + dc
            if 0 <= newr < R and 0 <= newc < C:
                if lines[newr][newc] in allowed[(dr, dc)]:
                    if (newr, newc) not in distances:
                        previous[(newr, newc)] = (r, c)
                        distances[(newr, newc)] = distances[(r, c)] + 1
                        q.append((newr, newc))
                    else:
                         if distances[(newr, newc)] > distances[(r, c)] + 1:
                            previous[(newr, newc)] = (r, c)
                            distances[(newr, newc)] = distances[(r, c)] + 1
                            q.append((newr, newc))
    return(max(distances.values()))

def part2():
    global distances

    pipes = set()
    for r in range(R):
        for c in range(C):
            if (r,c) in distances.keys():
                (er, ec) = (3*r+1, 3*c+1)
                pipe_type = lines[r][c]
                if pipe_type == 'S':
                    pipe_type = s_pipe()
                for dr, dc in expand[pipe_type]:
                    pipes.add((er+dr, ec+dc))

    no_pipes = set()
    for r in range(3*R):
        for c in range(3*C):
            if (r,c) not in pipes:
                no_pipes.add((r,c))

    in_set = set()
    while no_pipes:
        outside = False
        r,c = no_pipes.pop()
        q = deque([(r,c)])
        component = set()
        while q:
            r, c = q.popleft()
            for dr, dc in ((0, 1), (0, -1), (1, 0), (-1, 0)):
                newr, newc = r + dr, c + dc
                if 0 <= newr <= 3*R and 0 <= newc <= 3*C:
                    if (newr, newc) in no_pipes:
                        if newr == 0 or newr == 3*R or newc == 0 or newc == 3*C:
                            outside = True
                        no_pipes.remove((newr, newc))
                        component.add((newr, newc))
                        q.append((newr, newc))
        if not outside:
            in_set = in_set.union(component)

    res = 0
    for r in range(R):
        for c in range(C):
            if (3*r+1,3*c+1) in in_set:
                res += 1
    return res

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
