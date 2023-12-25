
# read command-line parameters and based on that read the input file
from collections import deque
from copy import deepcopy
from math import prod
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

lines = open(f"day19-{runtype}.txt", "r").readlines()
input = [i for i, line in enumerate(lines) if line == "\n"][0]
rulelines, partlines = [line.strip() for line in lines[:input]], [line.strip() for line in lines[input+1:]]


rules = {}
for rule in rulelines:
    name, rest = rule.split("{")
    rest = rest[:-1]
    rest = rest.split(",")
    rest = [r.split(":") for r in rest]
    rule_list = []
    for r in rest:
        if len(r) == 1:
            rule_list.append(r[0])
        else:
            rule_list.append([(r[0][0], r[0][1], int(r[0][2:])), r[1]])
    rest = rule_list
    rules[name] = rest

parts = []
for part in partlines:
    part = part[1:-1].split(",")
    xmas = {p.split("=")[0]: int(p.split("=")[1]) for p in part}
    parts.append(xmas)

def proces_part_with_rule(part, rule):
    for r in rule:
        if isinstance(r,str):
            return r
        else:
            rating = r[0][0]
            part_rating = part[rating]
            rule_rating = r[0][2]
            if (r[0][1] == '<' and part[r[0][0]] < r[0][2] ) or (r[0][1] == '>' and part[r[0][0]] > r[0][2] ):
                return r[1]

def process_part(part):
    cr = rules["in"]
    while True:
        p = proces_part_with_rule(part, cr)
        if p == 'A':
            return sum(part.values())
            break
        elif p == 'R':
            break
        else:
            cr = rules[p]
    return 0

def part1():
    res = 0
    for part in parts:
        res += process_part(part)
    return res

def new_range(operator, cutoff, values):
    result = deepcopy(values)
    if operator == '>':
        result[0] = max(values[0], cutoff+1)
    elif operator == '<':
        result[1] = min(values[1], cutoff-1)
    elif operator == '>=':
        result[0] = max(values[0], cutoff)
    elif operator == '<=':
        result[1] = min(values[1], cutoff)
    else:
        assert False
    return result

def new_ranges(rating, operator, cutoff, values):
    new_values = deepcopy(values)
    new_values[rating] = new_range(operator, cutoff, values[rating])
    return new_values

def find_ranges(rules):
    res = 0
    q = deque()
    q.append(("in", {'x': [1,4000], 'm': [1,4000], 'a': [1,4000], 's': [1,4000]}))

    while q:
        name, values = q.pop()

        # check if the values are still valid, if not skip this state
        if not all([v[0]<=v[1] for v in values.values()]):
            continue

        # if the state is value, calculate the total number of valid values and add to the result
        if name == 'A':
            res += prod([v[1]-v[0]+1 for v in values.values()])
            continue
        # if state is rejected skip it
        elif name == 'R':
            continue
        # otherwise process the rule
        else:
            # ge the rule
            rule = rules[name]
            # process all the conditions from left to right
            for condition in rule:
                # if it is really a condition (after my parsing this is always a list)
                if isinstance(condition, list):
                    rating   = condition[0][0]
                    operator = condition[0][1]
                    cutoff   = condition[0][2]

                    # add new state to queue for the condition that is met
                    q.append((condition[1], new_ranges(rating, operator, cutoff, values)))

                    # update the current values to the complementary condition
                    values = new_ranges(rating, '<=' if operator == '>' else '>=', cutoff, values)

                # no condition, just a new rule to be applied
                else:
                    q.append((condition, values))
                    break
    return res

def part2():
    res = find_ranges(rules)
    return res

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
