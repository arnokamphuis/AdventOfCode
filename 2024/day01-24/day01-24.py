
# read command-line parameters and based on that read the input file
import sys
from typing import Counter
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day01-{runtype}.txt", "r")

left_list, right_list = [], []
lines = [line.split() for line in text_file.readlines()]
for line in lines:
    left_list.append(int(line[0]))
    right_list.append(int(line[1]))

def part1():
    diffs = [abs(l-r) for l,r in zip(sorted(left_list), sorted(right_list))]
    return sum(diffs)

def part2():
    rc = Counter(right_list)
    similarity_score = [l*rc[l]  for l in left_list]    
    return sum(similarity_score)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
