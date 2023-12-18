
# read command-line parameters and based on that read the input file
from collections import defaultdict, deque
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day18-{runtype}.txt", "r")

lines = [line.split() for line in text_file.readlines()]
lines = [[line[0]] + [int(line[1])] + [line[2][2:-1]] for line in lines]
dir_codes = ['R', 'D', 'L', 'U']
dirs = {'U': (-1,0), 'D': (1,0), 'L': (0,-1), 'R': (0,1)}

def get_dir(index):
    return dirs[dir_codes[index]]

def calculate_integer_area(points):
    boundary_points_count = sum( \
        [abs(points[i-1][0] - points[i][0]) + abs(points[i-1][1] - points[i][1]) \
         for i in range(len(points))])
    
    # first use shoelace formula to calculate area of polygon
    # https://en.wikipedia.org/wiki/Shoelace_formula
    area = abs(sum( points[i][0] * (points[i-1][1] - points[(i+1) % len(points)][1]) for i in range(len(points)))) // 2

    # then use Pick's theorem to calculate area of polygon with integer coordinates
    # https://en.wikipedia.org/wiki/Pick%27s_theorem
    interior_points_count = area - boundary_points_count//2 + 1

    # finally, add the number of boundary points
    return interior_points_count + boundary_points_count

def part1():
    points = [(0,0)]
    for d, steps, _ in lines:
        dr, dc = dirs[d]
        r,c = points[-1]
        points.append((r + dr * steps, c + dc * steps))

    return calculate_integer_area(points)

def part2():
    points = [(0,0)]
    for _, _, code in lines:
        hex, dir_index = str(code[:5]), int(code[5])
        steps = int(hex,16)

        dr, dc = get_dir(dir_index)
        r,c = points[-1]
        points.append((r + dr * steps, c + dc * steps))
    return calculate_integer_area(points)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
