# read command-line parameters and based on that read the input file
from collections import defaultdict
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day12-{}.txt".format(runtype), "r")

lines = [line.split() for line in text_file.readlines()]
lines = [[line[0], [int(c) for c in line[1].split(',')]] for line in lines]

def get_counter(springs):
    count = 0
    counter = []
    for c in springs:
        if c == '#':
            count += 1
        elif c == '.':
            if count > 0:
                counter.append(count)
            count = 0
    if count > 0:
        counter.append(count)
    return counter

DP = defaultdict(int)
def dp(springs, checksum, spring_index, checksum_index, current_checksum_used):
    global DP

    key = (spring_index, checksum_index, current_checksum_used)
    if key in DP:
        return DP[key]
    
    if spring_index == len(springs):
        if checksum_index == len(checksum) and current_checksum_used == 0: # all springs placed and checksums match
            return 1
        elif checksum_index == len(checksum)-1 and checksum[checksum_index] == current_checksum_used: 
            return 1
        else:
            return 0

    res = 0
    # either place a spring or a dot
    for new_placement in ['#', '.']:
        if springs[spring_index] == new_placement or springs[spring_index] == '?':
            if new_placement == '.' and current_checksum_used == 0:
                # move to next character
                res += dp(springs, checksum, spring_index+1, checksum_index, 0)
            elif new_placement == '.' and current_checksum_used > 0 and checksum_index < len(checksum) and checksum[checksum_index] == current_checksum_used:
                # move to next character and checksum item
                res += dp(springs, checksum, spring_index+1, checksum_index+1, 0)
            elif new_placement == '#':
                # move to next character by placing a # out of the current checksum
                res += dp(springs, checksum, spring_index+1, checksum_index, current_checksum_used+1)
    DP[key] = res
    return res

def count_options(springs_collection):
    global DP
    res = 0
    for i, sc in enumerate(springs_collection):
        springs, checksum = sc
        DP.clear()
        ans = dp(springs, checksum, 0, 0, 0)
        res += ans
    return res

def part1():
    return count_options(lines)

def part2():
    sp = [[((springs+'?')*5)[:-1], checksum*5] for springs, checksum in lines]
    return count_options(sp)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
