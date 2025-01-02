
# read command-line parameters and based on that read the input file
from collections import defaultdict
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day04-{runtype}.txt", "r")

document = [[ch for ch in line.strip()] for line in text_file.readlines()]
R, C = len(document), len(document[0])

map = defaultdict(str)
for r in range(R):
    for c in range(C):
        map[(c,r)] = document[r][c]

def part1():
    dirs = [(i,j) for i in range(-1,2) for j in range(-1,2) if (i,j)!=(0,0) ]
    target = list('XMAS')
    count = 0
    for r in range(R):
        for c in range(C):
            for d in dirs:
                if [map[ (c+d[0]*l, r+d[1]*l)] for l in range(len(target))] == target:
                    count+=1
    return count

def part2():
    count = 0
    target = list('MMSS')
    elements = [(-1,-1), (-1,1), (1,1), (1,-1)]
    for r in range(R):
        for c in range(C):
            if map[(c,r)] == 'A':
                for i in range(len(elements)):
                    if [map[ (c+cc, r+rr)] for (rr,cc) in elements[i:] + elements[:i]] == target:
                        count+=1
                        break
    return count

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
