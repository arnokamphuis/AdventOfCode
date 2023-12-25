
# read command-line parameters and based on that read the input file
from collections import defaultdict
from functools import reduce
from math import gcd, prod
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day20-{runtype}.txt", "r")

lines = [line.split(' -> ') for line in text_file.readlines()]
lines = [[line[0], line[1].strip()] for line in lines]
lines = [ [
    [ line[0][0] if line[0][0] in '%&' else 'b', line[0][1:] if line[0][0] in '%&' else line[0]], 
    line[1].split(', ') ]   for line in lines]

modules = {}
for line in lines:
    modules[line[0][1]] = { 'type': line[0][0], 'output': line[1], 'state': None } 

for name, module in modules.items():
    if module['type'] == '%':
        module['state'] = 0
    if module['type'] == '&':
        module['state'] = {}
        for i, mod_in in modules.items():
            if name in mod_in['output']:
                module['state'][i] = 0

parents_of_rx = [n for n, m in modules.items() if 'rx' in m['output']]

parents_of_parents_of_rx = [
    n for n, m in modules.items() if any([p in parents_of_rx for p in m['output']])
]

previous = {}
count = defaultdict(int)
cycles = []

def press_button(modules, press_count, part):
    global previous, count, cycles
    signals_sent = [0,0]
    q = [('button', 'broadcaster', 0)]
    while len(q) > 0:
        f, t, signal = q.pop(0)

        if part == 2:
            if signal == 0:
                if t in previous and count[t] == 2 and t in parents_of_parents_of_rx:
                    cycles.append(press_count - previous[t])
                previous[t] = press_count
                count[t] += 1

            if len(cycles) == len(parents_of_parents_of_rx):
                return signals_sent, True

        fm, tm = None, None
        signals_sent[signal] += 1
        if f in modules:
            fm = modules[f]
        if t in modules:
            tm = modules[t]
        else:
            continue

        if tm['type'] == 'b':
            for o in tm['output']:
                q.append((t, o, signal))

        elif tm['type'] == '%':
            if signal == 1:
                continue
            else:
                tm['state'] = 1 - tm['state']
                for output in tm['output']:
                    signal_to_send = tm['state']
                    q.append((t, output, signal_to_send))

        elif tm['type'] == '&':
            tm['state'][f] = signal
            for output in tm['output']:
                signal_to_send = 0 if all([s == 1 for s in tm['state'].values()]) else 1
                q.append((t, output, signal_to_send))
        else:
            assert False
    return signals_sent, False


def part1():
    return prod(reduce(lambda x,y:  [x[0]+y[0], x[1]+y[1]], [press_button(modules, i, 1)[0] for i in range(1000)]))

def lcm(values):
    res = 1
    for v in values:
        res = (res*v) // gcd(v, res)
    return res

def part2():
    press = 1
    while True:
        signals_sent, done = press_button(modules, press, 2)
        press += 1
        if done:
            return lcm(cycles)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
