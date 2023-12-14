from copy import deepcopy
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day14-{}.txt".format(runtype), "r")

lines = [[c for c in line.strip()] for line in text_file.readlines()]

R = len(lines)
C = len(lines[0])

def move(rocks, dir):
    (dr, dc) = dir

    fR = 0 if dr == -1 else R-1
    tR = R if dr == -1 else -1
    fC = 0 if dc == -1 else C-1
    tC = C if dc == -1 else -1

    if dr != 0:
        for r in range(fR, tR, -dr):
            for c in range(C):
                if rocks[r][c] == 'O':
                    # move O to the last empty spot
                    rr =  r + dr
                    cc =  c + dc

                    while min(tR,fR) <= rr <= max(tR,fR) and rocks[rr][cc] == '.': 
                        rr += dr
                        cc += dc
                    rocks[r][c] = '.'
                    rocks[rr-dr][cc-dc] = 'O'
    else:
        for c in range(fC, tC, -dc):
            for r in range(R):
                if rocks[r][c] == 'O':
                    # move O to the last empty spot
                    rr =  r + dr
                    cc =  c + dc

                    while min(tC,fC) <= cc <= max(tC,fC) and rocks[rr][cc] == '.': 
                        rr += dr
                        cc += dc

                    rocks[r][c] = '.'
                    rocks[rr-dr][cc-dc] = 'O'
    return rocks

def calculate_load(rocks):
    res = 0
    for row in range(R):
        for col in range(C):
            if rocks[row][col] == 'O':
                res += R-row
    return res

def part1():
    return calculate_load(move(deepcopy(lines), (-1,0)))

def part2():
    dirs = [(-1,0), (0,-1), (1,0), (0,1)]

    rocks = deepcopy(lines)

    previous = [rocks]
    cycle = 0
    while True:
        for dir in dirs:
            rocks = move(deepcopy(rocks), dir)
        cycle += 1

        if rocks in previous:
            start_cycle = previous.index(rocks)
            current_cycle = cycle
            cycle_length = current_cycle - start_cycle
            remaining = 1000000000 - current_cycle
            remaining = remaining % cycle_length
            for j in range(0,remaining):
                for dir in dirs:
                    rocks = move(deepcopy(rocks), dir)
                cycle += 1
            break
        previous.append(rocks)

    return calculate_load(rocks)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
