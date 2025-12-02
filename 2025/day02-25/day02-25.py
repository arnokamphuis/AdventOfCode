
# read command-line parameters and based on that read the input file
import sys

runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day02-{runtype}.txt", "r")

lines = [line.split(',') for line in text_file.readlines()][0]
data = [list(map(int,line.split('-'))) for line in lines]

def find_factors(number):
    factors = []
    for i in range(1, int(number**0.5)+1):
        if number % i == 0:
            factors.append(i)
            if i != number // i:
                factors.append(number // i)
    return sorted(factors)

def find_invalid(number, part=1):
    num_str = str(number)
    n = len(num_str)
    
    for factor in find_factors(n)[:-1]:
        substrings = [num_str[i*factor:i*factor+factor] for i in range(n//factor)]
        if len(set(substrings)) == 1 and (part==2 or len(substrings)%2 == 0):
            return True
    return False

def sum_invalid(part):
    doubles = set()
    for d in data:
        for i in range(d[0],d[1]+1):
            if find_invalid(i, part=part):
                doubles.add(i)
    count = sum(doubles)
    return count

def part1():
    return sum_invalid(part=1)

def part2():
    return sum_invalid(part=2)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
