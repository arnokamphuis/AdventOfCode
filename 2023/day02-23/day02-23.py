from functools import reduce
import sys

runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day02-{}.txt".format(runtype), "r")

lines = [line.strip() for line in text_file.readlines()]
for line_index in range(len(lines)):
    lines[line_index] = lines[line_index].split(": ")
    lines[line_index][0] = lines[line_index][0][5:]
    lines[line_index][1] = lines[line_index][1].split("; ")
    lines[line_index][1] = [x.split(", ") for x in lines[line_index][1]]

def part1():
    res = 0
    bag = { "red": 12, "green": 13, "blue": 14 }
    for line in lines:
        possible = reduce(
            lambda x, y: x and y,
            [ reduce(
                lambda x, y: x and y,
                [bag[c.split(" ")[1]] >= int(c.split(" ")[0]) for c in turn]
                ) for turn in line[1]
            ])
        if possible:
            res += int(line[0])
    return res

def part2():
    res = 0
    for line in lines:
        game_power = { "red": 0, "green": 0, "blue": 0 }
        for turn in line[1]:
            for i in range(len(turn)):
                t = turn[i].split(" ")
                game_power[t[1]] = max(game_power[t[1]], int(t[0]))
        gp = 1
        for v in game_power.values():
            gp *= v
        res += gp
    return res

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
