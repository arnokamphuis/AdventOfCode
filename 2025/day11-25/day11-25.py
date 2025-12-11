
from functools import cache
import sys

runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day11-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]
nodes = {f: t.split(' ') for line in lines for f, t in [line.split(': ')]}

@cache
def count_all_paths(start, end, part=1, dac=False, fft=False):
    if (start == end and part == 1) or (start == end and dac and fft and part == 2):
        return 1
    else:
        if start not in nodes: return 0
        return sum(count_all_paths(n, end, part=part, dac=(dac or n=='dac'), fft=(fft or n=='fft')) for n in nodes[start])

def part1():
    return count_all_paths('you', 'out', part=1)

def part2():
    return count_all_paths('svr', 'out', part=2)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
