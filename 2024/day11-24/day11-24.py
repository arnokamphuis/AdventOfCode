
# read command-line parameters and based on that read the input file
import sys

runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day11-{runtype}.txt", "r")

numbers = text_file.readlines()[0].strip().split()
numbers = [int(n) for n in numbers]
original_numbers = numbers.copy()

DP = {}
def count_number_of_numbers(n, t):
    key = (n, t)
    if key in DP:
        return DP[key]
    
    if t == 0:
        return 1
    if n == 0:
        res = count_number_of_numbers(1, t-1)
    elif len(str(n))%2 == 0:
        nstr = str(n)
        left = int(nstr[:len(nstr)//2])
        right = int(nstr[len(nstr)//2:])
        res = count_number_of_numbers(left, t-1) + count_number_of_numbers(right, t-1)
    else:
        res = count_number_of_numbers(n*2024, t-1)
        
    DP[key] = res
    return res

def gen(nums, t):
    return sum([count_number_of_numbers(n,t) for n in nums])

def part1():
    return gen(numbers,25)

def part2():
    return gen(numbers, 75)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
