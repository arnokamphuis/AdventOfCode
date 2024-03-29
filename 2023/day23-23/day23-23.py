
# read command-line parameters and based on that read the input file
from collections import defaultdict, deque
from copy import deepcopy
import math
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day23-{runtype}.txt", "r")

grid = [[c for c in line.strip()] for line in text_file.readlines()]
R = len(grid)
C = len(grid[0])

start = (0, grid[0].index('.'))
end   = (R-1, grid[R-1].index('.'))

forces_steps = {
    'v': (1,0),
    '^': (-1,0),
    '>': (0,1),
    '<': (0,-1)
}

def neighbours(node):
    return [(node[0]+dr, node[1]+dc) for dr,dc in [(0,1),(0,-1),(1,0),(-1,0)] 
            if (0<=node[0]+dr<R and 0<=node[1]+dc<C) and (grid[node[0]+dr][node[1]+dc] != '#') and (grid[node[0]][node[1]] != '#')]

def count_neighbours(node):
    return len(neighbours(node))

nodes = [(r, c) for r in range(1,R-1) for c in range(1,C-1) if count_neighbours((r,c)) > 2] + [start, end]

def trace_path(pos, ppos):
    count = 1
    while pos not in nodes:
        ch = grid[pos[0]][pos[1]]
        if ch in 'v^<>':
            if (pos[0]+forces_steps[ch][0], pos[1]+forces_steps[ch][1]) == ppos:
                break
        (next,) = [n for n in neighbours(pos) if n != ppos]
        pos, ppos = next, pos
        count += 1
    return count, pos

distances = defaultdict(lambda: defaultdict(int))
for node in nodes:
    nb = neighbours(node)
    for next in nb:
        d, end_point = trace_path(next, node)
        if end_point in nodes:
            distances[node][end_point] = d

def solve(distances):
    visited = {node: False for node in nodes}

    def dfs(node, d):
        if node == end:
            return d

        res = -math.inf

        if visited[node]:
            return res
        
        visited[node] = True
        for opt, opt_d in distances[node].items():
            res = max(res, dfs(opt, d+opt_d))
        visited[node] = False
        return res

    return dfs(start, 0)

def part1():
    return solve(distances)

def part2():
    for node in nodes:
        for opt in nodes:
            if node!=opt and opt in distances[node].keys():
                distances[opt][node] = distances[node][opt]

    return solve(distances)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
