from z3 import *
# read command-line parameters and based on that read the input file
from collections import defaultdict
from copy import deepcopy
import heapq
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day10-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]

machines = []
for line in lines:
    bpos = line.find(']')
    light_status = sum([(1 if c == '#' else 0) << i for i, c in enumerate(line[1:bpos])])
    
    cbos = line.find('{')
    power = list(map(int, line[cbos+1:-1].split(',')))
    
    button_wiring = line[bpos+1:cbos].strip().split(' ')
    button_wiring = [list(map(int, bw[1:-1].split(','))) for bw in button_wiring]
    button_wiring = [sum([1 << v for v in bw])  for bw in button_wiring]
    machines.append({
        'light_status': light_status,
        'power': power,
        'button_wiring': button_wiring,
        'bits': bpos - 1
    })

def find_min(target, wiring, bits):
    visited = set()
    q = [(0, 0)]
    heapq.heapify(q)
    while q:
        cost, status = heapq.heappop(q)

        if status in visited:
            continue
        visited.add(status)
        
        if target == status:
            return cost

        for i in range(len(wiring)):
            button = wiring[i]
            btn_complement = ((1 << bits) - 1) - button
            status_inverted = ((1 << bits) - 1) - status
            
            new_status = (status & btn_complement) | (status_inverted & button)
            # print(f"  Pressing button {i} changes status to {new_status} (button: {button}, complement: {btn_complement}, inverted: {status_inverted})")
            heapq.heappush(q, (cost + 1, new_status))
    return -1

def part1():
    total = 0
    for machine in machines:
        total += find_min(machine['light_status'], machine['button_wiring'], machine['bits'])

    return total

def part2():
    total = 0
    for machine in machines:
        variables = []
        equations = []
        for i, bw in enumerate(machine['button_wiring']):
            variables.append(z3.Int(f'btn{i}'))

        optimizer = z3.Optimize()
        optimizer.minimize(sum(variables))
        for var in variables:
            optimizer.add(var >= 0)

        for i, pwr in enumerate(machine['power']):
            terms = []
            for j, bw in enumerate(machine['button_wiring']):
                if (bw >> i) & 1:
                    terms.append(variables[j])
            optimizer.add(sum(terms) == pwr)

        assert(optimizer.check()==z3.sat)

        model = optimizer.model()
        for v in variables:
            total += model[v].as_long()
    return total

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
