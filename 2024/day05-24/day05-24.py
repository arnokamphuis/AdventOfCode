from functools import cmp_to_key

# read command-line parameters and based on that read the input file
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day05-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]
cut = [i for i in range(len(lines)) if len(lines[i]) == 0][0]
page_ordering_rules = [list(map(int, line.split('|'))) for line in lines[:cut]]
pages_to_produce = [list(map(int, line.split(','))) for line in lines[cut+1:]]

def split_corret_incorrect(por, ptp):
    correct_lst = []
    incorrect_lst = []
    correct = True
    for page in ptp:
        correct = True
        for rule in por:
            if rule[0] in page and rule[1] in page:
                if page.index(rule[0]) > page.index(rule[1]):
                    correct = False
        if correct:
            correct_lst.append(page)
        else:
            incorrect_lst.append(page)
    
    return correct_lst, incorrect_lst

correct, incorrect = split_corret_incorrect(page_ordering_rules, pages_to_produce)    

def correct_cmp(x, y):
    for rule in page_ordering_rules:
        if x in rule and y in rule:
            if rule[0] == x:
                return -1
            else:
                return 1
    return 0

def part1():
    return sum([page[len(page) // 2] for page in correct])

def part2():
    corrected = [sorted(page, key=cmp_to_key(correct_cmp)) for page in incorrect]
    return sum([page[len(page) // 2] for page in corrected])

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
