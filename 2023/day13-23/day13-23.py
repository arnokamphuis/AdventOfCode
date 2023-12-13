
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day13-{}.txt".format(runtype), "r")

fields = [[[c for c in row] for row in map] for map in 
    [block.split('\n') for block in 
    [block for block in text_file.read().split('\n\n')]]]

def calculate_mirror_score(field, target):
    res = 0
    R = len(field)
    C = len(field[0])

    for c in range(1,C):
        if sum([len(smudge) for smudge in [[((r,c-1-cc), (r,c+cc)) for r in range(R) if field[r][c-1-cc] != field[r][c+cc]] for cc in range(0,min(C-c,c))]]) == target:
            res += c

    for r in range(1,R):
        if sum([len(smudge) for smudge in [[((r-1-rr,c), (r+rr,c)) for c in range(C) if field[r-1-rr][c] != field[r+rr][c]] for rr in range(0,min(R-r,r))]]) == target:
            res += r * 100
    return res

def part1():
    return sum([calculate_mirror_score(field, 0) for field in fields])

def part2():
    return sum([calculate_mirror_score(field, 1) for field in fields])

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
