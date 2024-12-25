
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day25-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]

codes = []
for i in range(0, len(lines), 8):
    pattern = lines[i:i+7]
    codes.append((pattern[0][0] == '.', [len([pattern[y][x] for y in range(7) if pattern[y][x] == "#"])-1 for x in range(5)]))

keys  = [c for (_,c) in list(filter(lambda x: x[0], codes))]
locks = [c for (_,c ) in list(filter(lambda x: not x[0], codes))]

# print(keys)
# print(locks)

def part1():
    res1 = 0
    for lock in locks:
        for key in keys:
            if all([key[i]+lock[i] <= 5 for i in range(len(key))]):
                res1 += 1
    return res1

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
    
