
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
    gates[t] = (i1, op, i2)


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

# Just reconstruct the full adder from the resulting bit gate
# All cases that are wrong will return the output wire that is wrong
def check_back(bit):
    global gates
    gate_name = f"z{bit:02d}"
    if gate_name not in gates: return False, gate_name, "not in gates"
    xor_gate = gates[gate_name]
    if xor_gate[1] != "XOR": return False, gate_name, "not XOR"
    
    in_wires = {xor_gate[0], xor_gate[2]}
    in_gates = {gn:gates[gn] for gn in in_wires}

    xor_gate_name = {gn for gn, gate in in_gates.items() if gate[1] == "XOR"}
    if len(xor_gate_name) == 0: 
        if bit == 1:
            return False, {gn for gn, gate in in_gates.items() if gate[1] != "AND"}.pop(), "initial bit and is wrong"
        else:
            return False, {gn for gn, gate in in_gates.items() if gate[1] != "OR "}.pop(), "carry is wrong"
    
    x_wire = f"x{bit:02d}"
    y_wire = f"y{bit:02d}"
    
    xor_gate_inputs = {(gate[0], gate[2]) for gn, gate in in_gates.items() if gate[1] == "XOR"}
    if len(xor_gate_inputs) != 1:
        incorrect_in_wires = [xgi for xgi in xor_gate_inputs if {xgi[0], xgi[1]} != {x_wire, y_wire}].pop()
        incorrect_in_wires = {incorrect_in_wires[0], incorrect_in_wires[1]}
        return False, [n for n, g in gates.items() if {g[0], g[2]} == incorrect_in_wires].pop(), "carry is wrong"

    if bit == 1:
        and_gate = {gate for gate in in_gates.values() if gate[1] == "AND"}.pop()
        if {and_gate[0], and_gate[2]} != {f"x00", f"y00"}:
            return False, {gn for gn, gate in in_gates.items() if gate[1] != "AND"}.pop(), "initial carry is wrong"
    else:
        or_gate = {gate for _, gate in in_gates.items() if gate[1] == "OR "}.pop()
        
        in_or_gate =  {(or_gate[0], gates[or_gate[0]]), (or_gate[2],gates[or_gate[2]])}
        in_or_gate_not_and = {n for (n,g) in in_or_gate if g[1] != "AND"}
        if len(in_or_gate_not_and) != 0:
            return False, in_or_gate_not_and.pop(), "and before carry is wrong"
    return True, None, "all good"


def part2():
    return ",".join(sorted(list({out for (_,out,_) in [check_back(i) for i in range(1, 45)]} - {None})))

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
