
# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day13-{}.txt".format(runtype), "r")

fields = [[[c for c in row] for row in map] for map in 
    [block.split('\n') for block in 
    [block for block in text_file.read().split('\n\n')]]]

def calculate_mirror_score(field):
    res = 0
    R = len(field)
    C = len(field[0])
    mirrors = []

    vertical_mirrors = []
    for c in range(1,C):
        if all([field[r][c-1-cc] == field[r][c+cc] for r in range(R) for cc in range(0,min(C-c,c))]):
            vertical_mirrors.append((c,C))
            mirrors.append(("col",c))
            res += c

    horizonal_mirrors = []
    for r in range(1,R):
        if all([field[r-1-rr][c] == field[r+rr][c] for c in range(C) for rr in range(0,min(R-r,r))]):
            horizonal_mirrors.append((r,R))
            mirrors.append(("row",r))
            res += 100 * r

    return res, mirrors

def part1():
    res = 0
    for findex, field in enumerate(fields):
        ans, mirrors = calculate_mirror_score(field)
        res += ans
    return res

def part2():
    res = 0
    for findex, field in enumerate(fields):
        R = len(field)
        C = len(field[0])

        all_smudges = []
        for c in range(1,C):
            smudges = [[((r,c-1-cc), (r,c+cc)) for r in range(R) if field[r][c-1-cc] != field[r][c+cc]] for cc in range(0,min(C-c,c))]
            count_smudges = sum([len(smudge) for smudge in smudges])
            if count_smudges == 1:
                smudges = list(filter(lambda smudge: len(smudge) == 1, smudges))[0][0]
                all_smudges.append(smudges)

        for r in range(1,R):
            smudges = [[((r-1-rr,c), (r+rr,c)) for c in range(C) if field[r-1-rr][c] != field[r+rr][c]] for rr in range(0,min(R-r,r))]
            count_smudges = sum([len(smudge) for smudge in smudges])
            if count_smudges == 1:
                smudges = list(filter(lambda smudge: len(smudge) == 1, smudges))[0][0]
                all_smudges.append(smudges)

        pre_ans, pre_mirrors = calculate_mirror_score(field)

        for smudges in all_smudges:
            from_r, from_c = smudges[0]
            to_r, to_c = smudges[1]
            field[to_r][to_c] = field[from_r][from_c]

        post_ans, post_mirrors = calculate_mirror_score(field)

        for post_mirror in post_mirrors:
            if post_mirror not in pre_mirrors:
                if post_mirror[0] == "row":
                    res += 100 * post_mirror[1]
                elif post_mirror[0] == "col":
                    res += post_mirror[1]

    return res

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
