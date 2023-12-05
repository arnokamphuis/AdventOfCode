
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day05-{}.txt".format(runtype), "r")

lines = [[ll.strip() for ll in line.split()] for line in text_file.readlines()]
seeds = [int(seed) for seed in lines[0][1:]]
allrules = []
all_interval_rules = []

def pairwise(iterable):
    a = iter(iterable)
    return zip(a, a)

seeds_iterval = [ (s, s+d)  for s, d in pairwise(seeds) ]

currentline = 1
while currentline < len(lines):
    rules = []
    interval_rules = []
    title = lines[currentline+1][0]
    currentline += 2
    while currentline < len(lines) and len(lines[currentline])>0:
        range_values = [int(n) for n in lines[currentline]]
        rules.append(range_values)
        interval_rules.append(
            [
                (int(range_values[1]), int(range_values[1]) + int(range_values[2])), 
                (int(range_values[0]), int(range_values[0]) + int(range_values[2]))
            ]
        )
        currentline += 1
    allrules.append(rules)
    all_interval_rules.append(interval_rules)

def part1():
    global seeds
    currentseeds = seeds
    for rules in allrules:
        newseeds = []
        for seed in currentseeds:
            notmapped = True
            for rule in rules:
                delta = seed - rule[1]
                if 0 <= delta <= rule[2]:
                    notmapped = False
                    newseeds.append(rule[0] + delta)
            if notmapped:
                newseeds.append(seed)
        currentseeds = newseeds
    return min(currentseeds)

def apply_ruleset(rules, currentseeds):
    seeds = []
    for (rule_src_start, rule_src_end), (rule_dst_start, rule_dst_end) in rules:
        newseeds = []
        while currentseeds:
            seed_start, seed_end = currentseeds.pop()
            delta = rule_dst_start - rule_src_start

            leftside = (seed_start, min(seed_end, rule_src_start))
            middle = (max(seed_start, rule_src_start), min(seed_end, rule_src_end))
            rightside = (max(rule_src_end, seed_start), seed_end)
            if leftside[1] > leftside[0]:
                newseeds.append(leftside)
            if middle[1] > middle[0]:
                seeds.append((middle[0]+delta, middle[1]+delta))
            if rightside[1] > rightside[0]:
                newseeds.append(rightside)
        currentseeds = newseeds
    return seeds + currentseeds

def part2():
    global seeds_iterval
    smallest = []
    for seed in seeds_iterval:
        currentseeds = [seed]
        for rules in all_interval_rules:
            seeds = []
            for (rule_src_start, rule_src_end), (rule_dst_start, rule_dst_end) in rules:
                newseeds = []
                while currentseeds:
                    seed_start, seed_end = currentseeds.pop()
                    delta = rule_dst_start - rule_src_start

                    leftside = (seed_start, min(seed_end, rule_src_start))
                    middle = (max(seed_start, rule_src_start), min(seed_end, rule_src_end))
                    rightside = (max(rule_src_end, seed_start), seed_end)
                    if leftside[1] > leftside[0]:
                        newseeds.append(leftside)
                    if middle[1] > middle[0]:
                        seeds.append((middle[0]+delta, middle[1]+delta))
                    if rightside[1] > rightside[0]:
                        newseeds.append(rightside)
                currentseeds = newseeds
            currentseeds = seeds + currentseeds
        smallest.append(min(currentseeds)[0])
    return min(smallest)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
