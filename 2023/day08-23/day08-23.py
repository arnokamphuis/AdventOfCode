
# read command-line parameters and based on that read the input file
from collections import defaultdict
from math import gcd
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day08-{}.txt".format(runtype), "r")

lines = [line.strip() for line in text_file.readlines()]
moves = [ 0 if c == 'L' else 1 for c in lines[0].strip() ]
n = len(moves)

nodes = lines[2:]

def least_common_multiple(xs):
  ans = 1
  for x in xs:
    ans = (x*ans)//gcd(x,ans)
  return ans

graph = defaultdict(list)
for node in nodes:
    f, t = node.split(" = ")
    graph[f] = t[1:-1].split(", ")

def part1():
    target = "ZZZ"
    current = "AAA"
    move = 0
    while current != target:
        current = graph[current][moves[ move % n ]]
        move += 1
    return move

def part2():
    all_A_nodes = [node for node in graph.keys() if node[2] == 'A']
    cycle_lengths = []

    for anode in all_A_nodes:
        move = 0
        current = anode
        while True:
            end = current[2]
            if end == 'Z':
                cycle_lengths.append(move)
                break
            else:
                current = graph[current][moves[ move % n ]]
                move += 1
    return least_common_multiple(cycle_lengths)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
