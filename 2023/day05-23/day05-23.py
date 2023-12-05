
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

currentline = 1
while currentline < len(lines):
    rules = []
    title = lines[currentline+1][0]
    currentline += 2
    while currentline < len(lines) and len(lines[currentline])>0:
        rules.append([int(n) for n in lines[currentline]])
        currentline += 1
    allrules.append(rules)

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


def part2():
    return 0

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
