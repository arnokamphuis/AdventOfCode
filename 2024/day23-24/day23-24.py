
# read command-line parameters and based on that read the input file
from collections import defaultdict
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day23-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]
connections = defaultdict(list)
computers = set()
for line in lines:
    f, t = line.split('-')
    computers.add(f)
    computers.add(t)
    connections[f].append(t)
    connections[t].append(f)
    
# print(connections)

computers = list(computers)

def create_strongly_cc(comps, connections):
    sets = []
    for comp in comps:
        sets.append(set(tuple(connections[comp])))
    targets = set.intersection(*sets).difference(comps)
    
    if len(targets) == 0:
        return comps
    
    for t in targets:
        new_set = comps.union([t])
        result = create_strongly_cc(new_set, connections)
        if result:
            return result
    
    return None

def find_all_strongly_cc(comps, connections):
    ccs = []
    for c1 in computers:
        for c2 in connections[c1]:
            if c2 == c1:
                continue
            test = set([c1,c2])
            result = create_strongly_cc(test, connections)
            if result:
                res = sorted(list(result))
                if res not in ccs:
                    ccs.append(res)
    return ccs

connected_components = find_all_strongly_cc(computers, connections)

def part1():    
    all_triples = set()
    for i in range(len(computers)):
        for j in range(i+1, len(computers)):
            if computers[j] in connections[computers[i]]:
                c1 = computers[i]
                c2 = computers[j]
                if c1[0] == 't' or c2[0] == 't':                
                    targets = set(connections[c1]).intersection(connections[c2]).difference([c1, c2])
                    for t in targets:
                        all_triples.add(tuple(sorted([t, c1, c2])))
    return len(all_triples)

def part2():
    max_size = max([len(cc) for cc in connected_components])
    largest = [cc for cc in connected_components if len(cc) == max_size]
    return ",".join(sorted(list(largest[0])))

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
