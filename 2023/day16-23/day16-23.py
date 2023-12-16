
# read command-line parameters and based on that read the input file
from collections import deque
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day16-{}.txt".format(runtype), "r")

lines = [line.strip() for line in text_file.readlines()]
lines = [[c for c in line] for line in lines]
R = len(lines)
C = len(lines[0])

grid = {}
for r in range(R):
    for c in range(C):
        grid[(r,c)] = lines[r][c]

def find_excited(grid, start):
    q = deque()
    visited = set()

    current, dir = start

    q.append((current, dir))
    while q:
        current, dir = q.popleft()
        if (current,dir) in visited:
            continue
        visited.add((current,dir))

        next = (current[0] + dir[0], current[1] + dir[1])
        if next not in grid:
            continue
        
        next_char = grid[next]

        if next_char == '.':
            q.append((next, dir))

        if next_char == '-':
            if dir in [(0,1), (0,-1)]:
                q.append((next, dir))
            else:
                q.append((next, (0,-1)))
                q.append((next, (0, 1)))

        if next_char == '|':
            if dir in [(1,0), (-1,0)]:
                q.append((next, dir))
            else:
                q.append((next, (-1,0)))
                q.append((next, ( 1,0)))

        if next_char == '\\':
            q.append((next, (dir[1], dir[0])))

        if next_char == '/':
            q.append((next, (-dir[1], -dir[0])))  

    return len(set([k[0] for k in visited])) - 1 # start pos outside is also counted

def part1():
    return find_excited(grid, ((0,-1), (0,1)))

def part2():
    excited = set()
    for r in range(R):
        excited.add(find_excited(grid, (( r,-1), ( 0, 1))))
        excited.add(find_excited(grid, (( r, C), ( 0,-1))))
    for c in range(C):
        excited.add(find_excited(grid, ((-1, c), ( 1, 0))))
        excited.add(find_excited(grid, (( R, c), (-1, 0))))

    return max(excited)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
