
# read command-line parameters and based on that read the input file
from collections import defaultdict
import sys
from networkx import minimum_cut, minimum_edge_cut, connected_components, Graph, DiGraph
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day25-{runtype}.txt", "r")

lines = [list(map(str.strip, line.split(': '))) for line in text_file.readlines()]
lines = { line[0]: line[1].split(' ') for line in lines}
connections = defaultdict(list)
for k,v in lines.items():
    for m in v:
        connections[k].append(m)
        connections[m].append(k)

# for c in connections:
#     print(c, connections[c])


def part1():
    graph = Graph()
    for n, cs in connections.items():
        for c in cs:
            graph.add_edge(n, c, capacity=1)
            graph.add_edge(c, n, capacity=1)
    
    edges_to_remove = minimum_edge_cut(graph)
    graph.remove_edges_from(edges_to_remove)
    comp1, comp2 = list(connected_components(graph))
    return len(comp1) * len(comp2)

def part2():
    return "Got 50 stars!!!! See you next year!"

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
