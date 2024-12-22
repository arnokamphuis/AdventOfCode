
# read command-line parameters and based on that read the input file
from collections import defaultdict
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day22-{runtype}.txt", "r")

numbers = list(map(int,[line.strip() for line in text_file.readlines()]))


def apply(secret):
    secret = ((secret * 64) ^ secret) % 16777216
    secret = ((secret // 32) ^ secret) % 16777216
    secret = ((secret * 2048) ^ secret) % 16777216
    return secret

def prices(secret):
    res = []
    for _ in range(2000):
        secret = apply(secret)
        res.append(secret)
    return res

def changes(prices):
    res = []
    for i in range(len(prices)-1):
        res.append(prices[i+1] - prices[i])
    return res

def values(prices_, changes_):
    res = {}
    for i in range(len(changes_)-3):
        pattern = tuple(changes_[i:i+4])
        if pattern not in res:
            res[pattern] = prices_[i+4]
    return res

def part1():
    sum = 0
    for secret in numbers:
        for _ in range(2000):
            secret = apply(secret)
        sum += secret
    return sum

def part2():
    all_scores = defaultdict(int)
    for secret in numbers:
        prices_ = prices(secret)
        real_prices_ = [p%10 for p in prices_]
        changes_ = changes(real_prices_)
        
        values_ = values(real_prices_, changes_)
        for change, value in values_.items():
            all_scores[change] += value
    return max(all_scores.values())

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
