
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day09-{}.txt".format(runtype), "r")

lines = [[int(x) for x in line.split()] for line in text_file.readlines()]

def predict(f, direction):
    deriv = [x-y for x,y in zip(f[1:], f[:-1])]
    if all(y==0 for y in deriv):
        return f[-1 if direction == 1 else 0]
    else:
        return f[-1 if direction == 1 else 0] + direction * predict(deriv, direction)

def part1():
    return sum([predict(line,  1) for line in lines])

def part2():
    return sum([predict(line, -1) for line in lines])

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
