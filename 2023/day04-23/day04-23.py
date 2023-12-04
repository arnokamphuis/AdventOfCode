
# read command-line parameters and based on that read the input file
from collections import defaultdict
from functools import reduce
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day04-{}.txt".format(runtype), "r")

cards = defaultdict(list)
count = defaultdict(int)
winningcount = defaultdict(int)

lines = [line.strip().split(": ") for line in text_file.readlines()]
for id_, cards_ in lines:
    id = int(id_[5:])
    wcards, ycards = cards_.split(" | ")
    winning = set(wcards.split())
    yours = set(ycards.split())
    wc = len(yours & winning)

    count[id] = 1
    winningcount[id] = wc

def part1():
    return reduce(
        lambda x, y: x + pow(2,y-1), 
        filter( lambda x: x > 0, winningcount.values()), 
        0)

def part2():
    for id, won in winningcount.items():
        for i in range(1,won+1):
            count[id+i] += count[id]
    return sum(count.values())

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
