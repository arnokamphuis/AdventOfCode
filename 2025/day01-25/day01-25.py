
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day01-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]

moves = [(line[0], int(line[1:])) for line in lines]

def part1():
    count = 0
    dial = 50
    max_dial = 100
    for move in moves:
        direction, steps = move
        if direction == 'R':
            dial = (dial + steps) % max_dial
        elif direction == 'L':
            dial = (dial - steps) % max_dial
        if dial == 0: count += 1
    return count

def part2():
    count = 0
    dial = 50
    max_dial = 100
    for move in moves:
        direction, steps = move
        extra = steps // max_dial
        steps = steps % max_dial

        new_dial = dial + (steps if direction == 'R' else -steps)
        if new_dial >= max_dial:
            count += extra + 1
            new_dial -= max_dial
        elif new_dial < 0:
            count += extra + (1 if not dial == 0 else 0)
            new_dial += max_dial
        else:
            count += extra + (1 if new_dial == 0 else 0)
        dial = new_dial
    return count

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
