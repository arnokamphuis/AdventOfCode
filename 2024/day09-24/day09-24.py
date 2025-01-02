
# read command-line parameters and based on that read the input file
from collections import defaultdict
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day09-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]
input = lines[0]

def part1():
    disk = []
    file_id = 0
    for i, c in enumerate(input):
        if i%2 == 0:
            for _ in range(int(c)):
                disk.append(file_id)
            file_id += 1
        else:
            for _ in range(int(c)):
                disk.append(-1)

    free_indices = [i for i in range(len(disk)) if disk[i] == -1]

    for fi in free_indices:
        while disk[-1] == -1: disk.pop()
        if len(disk) <= fi: break
        disk[fi] = disk.pop()

    res = sum([i*v for i, v in enumerate(disk) if v != -1])
    return res

def part2():
    files = {}
    free  = []
    file_id = 0
    pos = 0
    for i, c in enumerate(input):
        v = int(c)
        if i%2 == 0:
            files[file_id] = (pos, v)
            file_id += 1
        else:
            free.append((pos, v))
        pos += v

    while file_id > 0:
        # get the next file id from the end of the list
        file_id -= 1
        pos, size = files[file_id]
        # find the free space
        for i, (fpos, fsize) in enumerate(free):
            if fpos >= pos:
                # clean up the free space (all this free space is not needed)
                free = free[:i]
                break
            if size <= fsize: # if the free space is big enough
                # place the block in the free space
                files[file_id] = (fpos, size)

                # update the free space
                if size == fsize: # if this block of free space is fully used, remove it
                    free.pop(i)
                else: # make the free space smaller
                    free[i] = (fpos + size, fsize - size)
                break
    
    res2 = 0
    # calculate the result, go through all the files and calculate the value
    for file_id, (pos, size) in files.items():
        # for every position taken by the file, calculate the value
        for v in range(pos, pos + size):
            res2 += file_id * v

    return res2

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
