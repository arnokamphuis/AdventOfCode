# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day13-{runtype}.txt", "r")

lines = [line.split() for line in text_file.readlines()]

ngames = (len(lines)+1)//4

games = []

for g in range(ngames):
    game = []
    for i in range(2):
        x = int(lines[g*4+i][2][2:-1])
        y = int(lines[g*4+i][3][2:])
        game.append((x, y))

    x = int(lines[g*4+2][1][2:-1])
    y = int(lines[g*4+2][2][2:])
    game.append((x, y))
    games.append(game)

def solve(a, b, c, d, e, f, p2):
    if p2:
        e += 10000000000000
        f += 10000000000000
    
    x = (c*f - d*e) / (b*c - a*d)
    y = (e - a*x) / c
    
    if x%1 == 0 and y%1 == 0:
        return int(3*x + y)
    return 0

def part1():
    return sum([solve(a, b, c, d, e, f, False) for (a, b), (c, d), (e, f) in games])

def part2():
    return sum([solve(a, b, c, d, e, f, True) for (a, b), (c, d), (e, f) in games])

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
