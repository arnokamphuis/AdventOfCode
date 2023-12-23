
# read command-line parameters and based on that read the input file
from collections import defaultdict, deque
from copy import deepcopy
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

options = defaultdict(set)

for r in range(R):
    for c in range(C):
        option = set()
        if grid[r][c] != '#':
            if grid[r][c] in forces_steps.keys():
                option.add((r+forces_steps[grid[r][c]][0], c+forces_steps[grid[r][c]][1]))
            else:
                for dir in [(0,1),(0,-1),(1,0),(-1,0)]:
                    rr = r+dir[0]
                    rc = c+dir[1]
                    if 0 <= rr < R and 0 <= rc < C and grid[rr][rc] != '#':
                        option.add((rr,rc))
            options[(r,c)] = option

def trace_path(pos, ppos):
    count = 1
    while len([opt for opt in options[pos] if opt != ppos]) == 1:
        (next,) = [opt for opt in options[pos] if opt != ppos]
        ppos = pos
        pos = next
        count += 1
    return count, pos

distances = defaultdict(lambda: defaultdict(int))
nodes = [opt for opt in options if len(options[opt]) > 2] + [start,end]
for node in nodes:
    for opt in options[node]:
        if opt != node:
            d, end_point = trace_path(opt, node)
            if end_point in nodes:
                distances[node][end_point] = d

def solve(distances):
    visited = {node: False for node in nodes}

    res = 0
    def dfs(node, d):
        nonlocal res
        if visited[node]:
            return
        visited[node] = True
        if node == end:
            res = max(d,res)
        for opt, opt_d in distances[node].items():
            dfs(opt, d+opt_d)
        visited[node] = False

    dfs(start, 0)

    return res

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
    
