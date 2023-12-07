
# read command-line parameters and based on that read the input file
from collections import defaultdict
from functools import cmp_to_key
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open("day07-{}.txt".format(runtype), "r")

lines = [line.split() for line in text_file.readlines()]
cards = [(c, v) for c, v in lines]

valuecards = [
    dict(),
    dict([(v,i+2) for i, v in enumerate("23456789TJQKA")]),
    dict([(v,i+1) for i, v in enumerate("J23456789TQKA")])
]

def parse_card(card, part):
    d = defaultdict(int)
    for c in card:
        d[c] += 1

    if part == 2:
        # first, find the highest denomination in the card not being a joker J
        highest = list(d.keys())[0]
        for k in d.keys():
            if k != 'J':
                if d[k] > d[highest] or highest == 'J':
                    highest = k
        # if there is a joker J, add its count to the highest denomination
        # and remove the joker
        # only if the highest denomination is not a joker (because then all cards are jokers)
        if 'J' in d and highest != 'J':
            d[highest] += d['J']
            del d['J']

    if len(d) == 1: # five of a kind
        return 6
    if len(d) == 2: # four of a kind or full house
        if 4 in d.values():
            return 5
        else:
            return 4
    if len(d) == 3: # three of a kind or two pairs
        if 3 in d.values():
            return 3
        else:
            return 2
    if len(d) == 4: # one pair
        return 1
    else:
        return 0

def convert(card, part):
    value = ""
    for c in card:
        value += chr(ord('0') + valuecards[part][c])
    return value

def calc(part):
    valued_cards = [ ((parse_card(card[0],part), convert(card[0],part)),card[1]) for card in cards]
    sorted_cards = sorted(valued_cards, key=lambda x: x[0])
    res = 0
    for rank, card in enumerate(sorted_cards):
        res += (rank+1) * int(card[1])
    return res

def part1():
    return calc(1)

def part2():
    return calc(2)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
