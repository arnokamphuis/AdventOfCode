
# read command-line parameters and based on that read the input file
from itertools import product
from math import ceil, log10
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day07-{runtype}.txt", "r")

equations = [line.split() for line in text_file.readlines()]
equations = [[int(eq[0][:-1]), list(map(int,eq[1::]))] for eq in equations]

def solve(res, nums, concat = False):
    ops = ['+', '*']
    if concat:
        ops.append('||')
    n = len(nums)-1
    for op in list(product(ops, repeat=n)):
        t = nums[0]
        for i in range(0,n):
            if op[i] == '+':
                t += nums[i+1]
            elif op[i] == '*':
                t *= nums[i+1]
            elif op[i] == '||':
                t = pow(10,ceil(log10(nums[i+1])))*t + nums[i+1]
        if t == res:
            return True
    return False

def part1():
    res1 = 0
    for eq in equations:
        result = eq[0]
        numbers = eq[1::][0]
        if solve(result, numbers):
            res1 += result
    return res1

def part2():
    res2 = 0
    for eq in equations:
        result = eq[0]
        numbers = eq[1::][0]
        if solve(result, numbers, True):
            res2 += result
    return res2

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
