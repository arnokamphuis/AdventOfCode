
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day17-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]
regA = int(lines[0].split(" ")[2])
regB = int(lines[1].split(" ")[2])
regC = int(lines[2].split(" ")[2])

program_list = list(map(int, str(lines[4].split(" ")[1]).split(",")))
program = [program_list[i:i+2] for i in range(0, len(program_list), 2)]

def do_op(pc, program, part=1):
    global regA, regB, regC
    
    op = program[pc]
    
    inst = op[0]
    operand = op[1]

    value = None
    if operand < 4:    value = operand
    elif operand == 4: value = regA
    elif operand == 5: value = regB
    elif operand == 6: value = regC
    elif operand == 7: return 100000, "panic"
    
    out = None
    
    if inst == 0:
        if part == 1:
            num = regA
            den = 2**value
            regA = num // den
    elif inst == 1:
        regB = regB ^ operand            
    elif inst == 2:
        regB = value % 8
    elif inst == 3:
        if regA != 0:
            return operand, out
    elif inst == 4:
        regB = regB ^ regC
    elif inst == 5:
        out = value % 8
        if part == 2:
            return None, out
    elif inst == 6:
        num = regA
        den = 2**value
        regB = num // den
    elif inst == 7:
        num = regA
        den = 2**value
        regC = num // den            

    return 2*pc + 2, out

def execute(program):
    global regA, regB, regC
    pc = 0
    output = []
    while pc < len(program):
        adv, out = do_op(pc, program)
        if out != None:
            output.append(out)
        pc = adv//2
    return ",".join(map(str, output))


def find(target, ans):
    global regA, regB, regC

    # when exeucting the whole program, the output is generating a program
    # in the meantime regA is getting smaller and smaller (by 3 bits)
    # therefore, the last cycle of the program will generate the last step
    # in the new program, based on a value between 0 and 7
    # we now need to search backwards throught the existing program to find
    # sets of three bits (in front of the existing bits in regA) that will
    # generate the next values (from the end of the program to the beginning)
    # we do this recursively until we find the first three bits that will
    # generate the first value in the new program

    if target == []: 
        return ans
    
    for t in range(8):
        regA = (ans << 3) + t
        regB = 0
        regC = 0
        out = None
        
        for pc in range(0, len(program)):
            _, out = do_op(pc, program, 2)
            if out is not None:
                if out == target[-1]:
                    ans_rest = find(target[:-1], regA)
                    if ans_rest is None: 
                        continue
                    return ans_rest

def part1():
    return execute(program)


def part2():
    return find(program_list, 0)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
