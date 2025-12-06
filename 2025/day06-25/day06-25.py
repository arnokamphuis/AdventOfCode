
# read command-line parameters and based on that read the input file
import sys

from math import prod
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day06-{runtype}.txt", "r")

lines = [line for line in text_file.readlines()]

def part1():
    numbers = [] * (len(lines)-1)
    operations = lines[-1].split()
    for input_idx in range(len(lines)-1):
        numbers.append(list(map(int, lines[input_idx].split())))

    total = 0
    number_of_problems = len(numbers[0])
    for problem_idx in range(number_of_problems):
        problem_numbers = [numbers[input_idx][problem_idx] for input_idx in range(len(lines)-1)]
        if operations[problem_idx] == '+':
            result = sum(problem_numbers)
        elif operations[problem_idx] == '*':
            result = prod(problem_numbers)
        total += result
    return total    

def part2():
    def find_next_start(opeerations, idx):
        idx += 1
        while idx < len(operations) and operations[idx] == ' ':
            idx += 1
        if idx == len(operations):
            return idx+1
        return idx

    number_count = len(lines) - 1
    
    idx = 0
    operations = lines[-1]

    total = 0
    while True:
        next_idx = find_next_start(operations, idx) - 1
        if idx >= len(operations):
            break

        grid = []
        for digit_idx in range(number_count):
            n = [lines[digit_idx][i] if (lines[digit_idx][i] != ' ') else '0' for i in range(idx, next_idx) ]
            grid.append(n)
        transposed = list(map(list, zip(*grid)))
        numbers = [  int(''.join([c for c in row if c != '0'])) for row in transposed ]

        if operations[idx] == '+':
            result = sum(numbers)
        elif operations[idx] == '*':
            result = prod(numbers)
        total += result

        idx = next_idx+1
    return total

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
