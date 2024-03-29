
# read command-line parameters and based on that read the input file
import heapq
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day17-{runtype}.txt", "r")

energy_loss = [[int(c) for c in map(str.strip,line)] \
               for line in map(str.strip,text_file.readlines())]


def find_minimum_energyloss(energy_loss, part):
    res = 0
    R, C = len(energy_loss), len(energy_loss[0])
    dirs = { 0: (-1,0), 1: (0,1), 2: (1,0), 3: (0,-1) }

    # state is (energy, (row, col), direction, last_sequence_length)
    # energy is the total energy loss so far, 
    #   that is the cost, therefore also the priority in the queue
    # this way you will always find the minimum energy loss at 
    # first visited (in a certain direction)
    # start in the direction -1 with sequence length -1 making
    # sure that the first step is always valid

    q = [(R+C-2, (0, 0), -1, -1, 0)]
    visited = {}
    while q:
        _, (cr,cc), cd, lseq, energy = heapq.heappop(q)

        if (cr,cc) == (R-1,C-1) and ((part == 1) or (part == 2 and lseq >= 4)):
            res = energy
            break
        
        key = ((cr,cc), cd, lseq)
        if key in visited:
            continue
        visited[key] = energy

        for dd in [-1, 0, 1] if cd != -1 else [2, 3]:
            nd = (cd + dd) % 4
            ddir = dirs[nd]
            nr, nc = cr + ddir[0], cc + ddir[1]
            nlseq = (1 if nd != cd else lseq + 1)

            valid_steps = \
                (part == 1 and (nlseq <= 3)) or \
                (part == 2 and ((nd != cd and lseq >= 4) or (nd==cd and nlseq <= 10) or lseq == -1))

            if 0 <= nr < R and 0 <= nc < C and valid_steps:
                ne = energy + energy_loss[nr][nc]
                heur = ne + (R-1-nr) + (C-1-nc)
                heapq.heappush(q, (heur, (nr, nc), nd, nlseq, ne))

    return res


def part1():
    return find_minimum_energyloss(energy_loss, 1)

def part2():
    return find_minimum_energyloss(energy_loss, 2)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
