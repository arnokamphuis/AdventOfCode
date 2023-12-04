# read command-line parameters and based on that read the input file
from functools import reduce
import sys
from collections import defaultdict 

runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day03-{}.txt".format(runtype), "r")

lines = [[c for c in line] for line in text_file.readlines()]
lastline = lines[-1]
lines = [line[:-1] for line in lines[:-1]]
lines.append(lastline)

delta = [(-1,-1), (-1,+1), (+1,-1), (+1,+1), (-1,0), (1,0), (0,-1), (0,+1)]
C = len(lines)
R = len(lines[0])


def part1():
    res = 0
    for i in range(R):
        current_number = 0
        ispart = False
        for j in range(C+1):
            if j < C and lines[i][j].isdigit():
                current_number = current_number * 10 + int(lines[i][j])

                for d in delta:
                    x = i + d[0]
                    y = j + d[1]
                    if x >= 0 and x < R and y >= 0 and y < C:
                        if not lines[x][y].isdigit() and lines[x][y] != '.':
                            ispart = True
            else:
                if ispart:
                    res += current_number
                current_number = 0
                ispart = False
    return res

def part2():
    res = 0
    gear_numbers = defaultdict(list)
    for i in range(R):
        current_number = 0
        isgear = set()
        for j in range(C+1):
            if i<R and j < C and lines[i][j].isdigit():
                current_number = current_number * 10 + int(lines[i][j])

                for d in delta:
                    x = i + d[0]
                    y = j + d[1]
                    if x >= 0 and x < R and y >= 0 and y < C:
                        if lines[x][y] == '*':
                            isgear.add((x,y))

            else:
                if len(isgear)>0:
                    for gear in isgear:
                        gear_numbers[gear].append(current_number)
                current_number = 0
                isgear = set()

    return sum(
        map(lambda g: g[0]*g[1], 
            filter(lambda g: len(g) == 2, 
                   gear_numbers.values()
                   )
            )
        )

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
