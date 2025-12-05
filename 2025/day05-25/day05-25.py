
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day05-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]
idx_empty = lines.index("")

fresh_ranges = [list(map(int,line.split('-')))for line in lines[:idx_empty]]
ids = [int(line) for line in lines[idx_empty+1:]]

def part1():
    count = 0
    for id in ids:
        if any([l <= id <= r for [l,r] in fresh_ranges]):
            count += 1
    return count

def merge_ranges(range1, range2):
    l1, r1 = range1
    l2, r2 = range2
    if r1 < l2 - 1 or r2 < l1 - 1:
        return [range1, range2] # no overlap, return both
    return [[min(l1, l2), max(r1, r2)]]

def part2():
    global fresh_ranges
    merged_ranges = []
    while True:
        new_range = fresh_ranges.pop(0)
        changed = False
        for i in range(len(merged_ranges)):
            result = merge_ranges(new_range, merged_ranges[i])
            if len(result) == 1:
                new_range = result[0]
                merged_ranges.pop(i)
                changed = True
                break
        merged_ranges.append(new_range)
        if not fresh_ranges:
            if not changed:
                break
            fresh_ranges = merged_ranges
            merged_ranges = []
    
    sum_ranges = sum([r-l+1 for [l,r] in merged_ranges])
    return sum_ranges

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
