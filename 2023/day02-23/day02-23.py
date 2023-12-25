from math import prod
from collections import defaultdict
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
    lines[line_index][1] = [turns.split(", ") for turns in lines[line_index][1]]
    lines[line_index][1] = [[turn.split() for turn in turns] for turns in lines[line_index][1]]


def part1():
    bag = { "red": 12, "green": 13, "blue": 14 }
    return reduce(
        lambda x, y: x + y,
        [
            int(id) if reduce(
            lambda x, y: x and y,
            [ reduce(
                lambda x, y: x and y,
                [bag[color] >= int(count) for count, color in turn]
                ) for turn in turns
            ]) else 0
            for id, turns in lines
        ],
        0
    )

def part2():
    return sum([
        prod ( 
            reduce(
                lambda x, y: { color: max(x.get(color,0), y.get(color,0)) for color in x },
                [ dict([ (color, int(count)) for count, color in turn]) for turn in turns],
                { "red": 0, "green": 0, "blue": 0 }
            ).values() 
        ) for _, turns in lines    
    ])

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
