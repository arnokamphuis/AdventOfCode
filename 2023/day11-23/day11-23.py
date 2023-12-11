
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day11-{}.txt".format(runtype), "r")

lines = [[c for c in line.strip()] for line in text_file.readlines()]
R = len(lines)
C = len(lines[0])

galaxies = []
galaxynumber = {}
for r in range(R):
    for c in range(C):
        if lines[r][c] == '#':
            galaxynumber[(r,c)] = len(galaxies)+1
            galaxies.append((r,c))

free_row = []
free_col = []
for r in range(R):
    if  all([galaxy[0] != r for galaxy in galaxies]):
        free_row.append(r)
for c in range(C):
    if all([galaxy[1] != c for galaxy in galaxies]):
        free_col.append(c)

def calculate_distances(expansion):
    res = 0
    for g1 in galaxies:
        for g2 in galaxies:
            if g1 == g2:
                continue
            else:
                dr = abs(g2[0] - g1[0])
                dc = abs(g2[1] - g1[1])
                top = min(g1[0], g2[0])
                bottom = max(g1[0], g2[0])
                left = min(g1[1], g2[1])
                right = max(g1[1], g2[1])
                dr += (expansion-1)*len(list(filter(lambda x: x > top  and x < bottom, free_row)))
                dc += (expansion-1)*len(list(filter(lambda x: x > left and x < right, free_col)))
                res += dr+dc
    return res//2

def part1():
    return calculate_distances(2)

def part2():
    return calculate_distances(1000000)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
