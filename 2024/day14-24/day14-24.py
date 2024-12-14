from math import prod
import re
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day14-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]

regex = r"[-+]?\d+"

robots = [ list(map(int, re.findall(regex, line))) for line in lines]


robots = [((robot[0],robot[1]), (robot[2],robot[3])) for robot in robots]

# for robot in robots:
#     print(robot)

if runtype == "test":
    C, R = 11, 7
else:
    C, R = 101, 103

# robots = [robot for robot in robots if robot[0]==(2,4) and robot[1]==(2,-3)]

# print(robots)    

def get_positions(time):
    positions = []
    for r in robots:
        x, y = r[0]
        vx, vy = r[1]
        current_pos = ((x + vx * time)%C, (y + vy * time)%R)
        positions.append(current_pos)
    return positions

def get_quadrant(pos, middle):
    if pos[0] < middle[0] and pos[1] < middle[1]:
        return 0
    elif pos[0] > middle[0] and pos[1] < middle[1]:
        return 1
    elif pos[0] < middle[0] and pos[1] > middle[1]:
        return 2
    elif pos[0] > middle[0] and pos[1] > middle[1]:
        return 3
    else:
        return 4

def part1():
    target_time = 100
    positions = get_positions(target_time)
        
    middle = (C//2, R//2)
    quadrants = [0,0,0,0,0]
    for pos in positions:
        quadrants[get_quadrant(pos, middle)] += 1
    return prod(quadrants[0:-1])

def count_largest_ccs(positions):
    dirs = [(-1,0), (1,0), (0,-1), (0,1)]
    largest_ccs = 0
    visited = set()
    for pos in positions:
        if pos in visited:
            continue
        cc_count = 0
        stack = [pos]
        while stack:
            current = stack.pop()
            cc_count += 1
            visited.add(current)
            for dc,dr in dirs:
                neighbour = (current[0]+dc, current[1]+dr)
                if neighbour in positions and neighbour not in visited:
                    stack.append(neighbour)
        largest_ccs = max(largest_ccs, cc_count)
    return largest_ccs

def part2():
    t = 0
    while True:
        t+=1
        max_cc = count_largest_ccs(get_positions(t))
        if max_cc > 200:
            break        
    return t

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
