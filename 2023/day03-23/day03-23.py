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


def get_value(grid, x, y):
    if x < 0 or x >= len(grid):
        return '.'
    if y < 0 or y >= len(grid[0]):
        return '.'
    return grid[x][y]

def create_value(grid,s):
    i = 0
    res = 0
    for digit in s:
        v = get_value(grid, digit[0], digit[1])
        res += pow(10,i) * int(v)
        i += 1
    return res

def find_value(grid, coord):
    x,y = coord
    res = int(get_value(grid, x, y))

    x,y = coord
    offset = 0
    while get_value(grid, x, y-1).isdigit():
        y -= 1
        offset += 1
        res += pow(10,offset) * int(get_value(grid, x, y))

    x,y = coord
    while get_value(grid, x, y+1).isdigit():
        y += 1
        res = 10 * res + int(get_value(grid, x, y))

    return res

def part1():

    numbers = []
    digit_coordinates = set()
    for i in range(C):
        for j in range(R):
            if lines[i][j].isdigit():
                digit_coordinates.add((i,j))

    while len(digit_coordinates) > 0:
        num = set()
        x,y = digit_coordinates.pop()
        # print(x,y)
        num.add((x,y))
        dy = 1
        if (x,y+dy) in digit_coordinates:
            while (x,y+dy) in digit_coordinates:
                num.add((x,y+dy))
                digit_coordinates.remove((x,y+dy))
                dy += 1
        else:
            dy = -1
            while (x,y+dy) in digit_coordinates:
                num.add((x,y+dy))
                digit_coordinates.remove((x,y+dy))
                dy -= 1
        numbers.append(num)

    partnumbers = []
    for num in numbers:
        for x,y in num:
            for d in delta:
                v = get_value(lines, x+d[0], y+d[1])
                if not (v.isdigit() or v == '.') and num not in partnumbers:
                    partnumbers.append(num)

    real_parts = [ create_value(lines, part) for part in partnumbers ]

    return reduce( lambda x,y: x + y, real_parts)

def part2():
    res = 0
    for i in range(R):
        for j in range(C):            
            found = 0
            coordinates_found = []
            if lines[i][j] == '*':

                if get_value(lines, i-1, j-1).isdigit() and get_value(lines, i-1, j) == '.' and get_value(lines, i-1, j+1).isdigit():
                    coordinates_found.append((i-1,j-1))
                    coordinates_found.append((i-1,j+1))
                else:
                    if get_value(lines, i-1, j).isdigit():
                        coordinates_found.append((i-1,j))
                    elif get_value(lines, i-1, j-1).isdigit():
                        coordinates_found.append((i-1,j-1))
                    else:
                        if get_value(lines, i-1, j+1).isdigit():
                            coordinates_found.append((i-1,j+1))
                        
                if get_value(lines, i+1, j-1).isdigit() and get_value(lines, i+1, j) == '.' and get_value(lines, i+1, j+1).isdigit():
                    coordinates_found.append((i+1,j-1))
                    coordinates_found.append((i+1,j+1))
                else:
                    if get_value(lines, i+1, j).isdigit():
                        coordinates_found.append((i+1,j))
                    elif get_value(lines, i+1, j-1).isdigit():
                        coordinates_found.append((i+1,j-1))
                    else:
                        if get_value(lines, i+1, j+1).isdigit():
                            coordinates_found.append((i+1,j+1))
                
                if get_value(lines, i, j-1).isdigit():
                    coordinates_found.append((i,j-1))
                if get_value(lines, i, j+1).isdigit():
                    coordinates_found.append((i,j+1))

                if len(coordinates_found)==2:
                    gear_power = find_value(lines, coordinates_found[0]) * find_value(lines, coordinates_found[1])
                    res += gear_power
    return res

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
