
# read command-line parameters and based on that read the input file
from collections import defaultdict
from itertools import product
import random
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day24-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]
empty = lines.index('')
inputs = { line[0:3]: int(line[4:]) for line in lines[:empty] }

gates = {}
for line in lines[empty+1:]:
    f, t = line.split(' -> ')
    if 'OR' in f:
        f = f.replace(' OR', ' OR ')
    i1 = f[0:3]
    op = f[4:7]
    i2 = f[8:11]
    gates[t] = [i1, op, i2]


def process(wires):
    global gates
    active = {out:op for (out,op) in gates.items() if op[0] in wires and op[2] in wires}

    output = {}
    for out, gate in active.items():
        i1 = wires[gate[0]]
        i2 = wires[gate[2]]
        if gate[1] == 'AND':
            output[out] = i1 & i2
        elif gate[1] == 'OR ':
            output[out] = i1 | i2
        elif gate[1] == 'XOR':
            output[out] = i1 ^ i2
        else:
            raise Exception("Unknown operator")
    return output

def get_decimal_value(wires):
    v = 0
    for i in range(len(wires)-1, -1, -1):
        z_str = '{:02d}'.format(i)
        for name, value in wires.items():
            if name.endswith(z_str):
                v |= value << i
    return v

def process_until_target_are_set(wires, target):
    global gates
    target_values = {t: None for t in target}
    loops = 0
    while any([v == None for v in target_values.values()]) and loops < 100:
        output = process(wires)
        wires.update(output)
        for t in target:
            if t in output:
                target_values[t] = wires[t]
        loops += 1
        
    if loops == 100:
        return None
    return target_values

def part1():
    global gates
    z_gates = {k: v for k, v in gates.items() if k.startswith('z')}
    z_values = process_until_target_are_set(inputs, z_gates.keys())
    res1 = get_decimal_value(z_values)
    return res1
    return 0

def check_recarry(wire, i):
    global gates
    if wire not in gates: return False
    parent_gate = gates[wire]
    
    if parent_gate[1] != "AND": return False
    return check_inter_xor(parent_gate[0], i) and check_carry_bit(parent_gate[2], i) or \
        check_inter_xor(parent_gate[2], i) and check_carry_bit(parent_gate[0], i)

def check_direct_carry(wire, i):
    global gates
    if wire not in gates: return False
    parent_gate = gates[wire]
    
    if parent_gate[1] != "AND": return False
    
    return sorted([parent_gate[0], parent_gate[2]]) == [f"x{i:02d}", f"y{i:02d}"]

def check_carry_bit(wire, i):
    global gates
    if wire not in gates: return False
    parent_gate = gates[wire]
    
    if i == 1:
        if parent_gate[1] != "AND": return False
        return sorted([parent_gate[0], parent_gate[2]]) == ["x00", "y00"]
    
    if parent_gate[1] != "OR ": return False
    
    return (check_direct_carry(parent_gate[0], i - 1) and check_recarry(parent_gate[2], i - 1) ) or \
        check_direct_carry(parent_gate[2], i - 1) and check_recarry(parent_gate[0], i - 1)

def check_inter_xor(wire, i):
    global gates
    if wire not in gates: return False
    parent_gate = gates[wire]
    
    if parent_gate[1] != "XOR": return False
    
    return sorted([parent_gate[0], parent_gate[2]]) == [f"x{i:02d}", f"y{i:02d}"]

def check_back_track(wire, i):
    global gates
    if wire not in gates: return False
    parent_gate = gates[wire]
    
    if parent_gate[1] != "XOR": return False
    
    if i==0: return sorted([parent_gate[0], parent_gate[2]]) == ['x00', 'y00']
    
    return (check_inter_xor(parent_gate[0], i) and check_carry_bit(parent_gate[2], i)) or \
        (check_inter_xor(parent_gate[2], i) and check_carry_bit(parent_gate[0], i))

def check(i):
    return check_back_track(f"z{i:02d}", i)

def where_okay():
    i = 0
    while True:
        if not check(i): break
        i += 1
    return i

def swap_gates(x, y):
    global gates
    gates[x], gates[y] = gates[y], gates[x]

def part2():
    global gates
    swaps = []
    for _ in range(4):
        print("progress")
        progress = where_okay()
        print(progress)
        for g1 in gates:
            for g2 in gates:
                if g1 == g2: continue
                # print(g1, g2)
                swap_gates(g1, g2)
                if where_okay() > progress:
                    print(f"Swapping {g1} with {g2}")
                    break
                swap_gates(g1, g2)
            else:
                continue
            break
        swaps.extend([g1, g2])
    print(swaps)
    return ",".join(sorted(swaps))

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
