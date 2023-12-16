
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

lines = [[c for c in line] for line in map(str.strip, text_file.readlines())]
grid = {(r,c): lines[r][c] for r in range(len(lines)) for c in range(len(lines[0]))}

def find_excited(start):
    global grid
    q = deque()
    visited = set()

    q.append(start)
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

    visited.remove(start)

    return len(set([k[0] for k in visited]))

def part1():
    return find_excited(((0,-1), (0,1)))

def part2():
    res = 0
    
    R = len(lines)
    C = len(lines[0])

    for r in range(R):
        res = max(res, 
                  find_excited(((r,-1), (0, 1))), 
                  find_excited(((r, C), (0,-1))))
    for c in range(C):
        res = max(res, 
                  find_excited(((-1, c), ( 1, 0))), 
                  find_excited((( R, c), (-1, 0))))

    return res

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
