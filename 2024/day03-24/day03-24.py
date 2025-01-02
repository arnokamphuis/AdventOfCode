
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day03-{runtype}.txt", "r")

instructions = ''.join(text_file.readlines()).strip()

def find_next_valid_operation(str):
    n1s = n2s = n1e = n2e = None
    index = 0
    if str[index:index+4] == 'mul(':
        index += 4
        
        n1s = index
        while str[index] in '0123456789':
            index += 1
        if str[index] != ',':
            return n1s, None
        n1e = index
        index += 1
        
        n2s = index
        while str[index] in '0123456789':
            index += 1
        if str[index] != ')':
            return n2s, None
        n2e = index
    else:
        return 1, None
    return n2e+1, (int(str[n1s:n1e]), int(str[n2s:n2e]))

def execute(instr, enabler=False):
    index = 0
    do = True
    ans = 0 
    while index < len(instructions):
        if instructions[index:index+4] == 'do()':
            do = True
            index += 4
        if instructions[index:index+7] == 'don\'t()':
            do = False
            index += 7
        next_index, instr = find_next_valid_operation(instructions[index:])
        index += next_index
        if instr is not None:
            a,b = instr
            if do or not enabler:
                ans += a*b
    return ans

def part1():
    return execute(instructions, False)

def part2():
    return execute(instructions, True)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
