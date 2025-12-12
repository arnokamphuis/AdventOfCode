
# read command-line parameters and based on that read the input file
from copy import deepcopy
from functools import cache
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day12-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]
blank_lines = [-1] + [idx for idx, val in enumerate(lines) if val == '']  # just to show how to get blank lines if needed

presents = []
for i in range(len(blank_lines)-1):
    present = []
    for (r, line) in enumerate(lines[blank_lines[i]+2:blank_lines[i+1]]):
        for (c, x) in enumerate(line):
            if x == '#':
                present.append((r, c))
    presents.append(present)


shapes = []
for p in presents:
    unique_variants = []
    seen = set()
    present = deepcopy(p)
    for _ in range(2):  # flip
        for _ in range(4):  # rotate
            min_r = min([p[0] for p in present])
            min_c = min([p[1] for p in present])
            normed = sorted([(p[0]-min_r, p[1]-min_c) for p in present])
            normed_tuple = tuple(normed)
            if normed_tuple in seen:
                # already seen
                pass
            else:
                seen.add(normed_tuple)
                max_r = max([p[0] for p in normed])
                max_c = max([p[1] for p in normed])
                width = max_c + 1
                height = max_r + 1

                unique_variants.append(
                    {
                        'shape': normed,
                        'width': width,
                        'height': height
                    }
                )
            # rotate 90 degrees
            present = [(p[1], -p[0]) for p in present]
        # flip vertically
        present = [(-p[0], p[1]) for p in present]
    shapes.append({'index': len(shapes), 'variants': unique_variants})

regions = []
trees = lines[blank_lines[-1]+1:]
for tree in trees:
    size, amounts = tree.split(': ')
    amounts = list(map(int, amounts.split(' ')))
    w, h = list(map(int, size.split('x')))
    region = {'size': (w, h), 'amounts': amounts}
    regions.append(region)

def solve(region, shapes):
    # implement a backtracking solution to fit all shapes in the region
    w, h = region['size']

    total_area_of_presents = 0
    area_map = {}
    for (idx, amount) in enumerate(region['amounts']):
        shape = shapes[idx]
        area = len(shape['variants'][0]['shape'])
        area_map[idx] = area
        total_area_of_presents += area * amount
    if total_area_of_presents > w * h:
        return False  # impossible

    order = []
    for (idx, amount) in enumerate(region['amounts']):
        for _ in range(amount):
            order.append(idx)

    def placements_for_shape(shape_idx):
        placements = []
        seen = set()
        for variant in shapes[shape_idx]['variants']:
            vh = variant['height']
            vw = variant['width']
            coords = variant['shape']
            for r in range(h - vh + 1):
                for c in range(w - vw + 1):
                    mask = 0
                    for (dr, dc) in coords:
                        rr = r + dr
                        cc = c + dc
                        pos = rr * w + cc
                        mask |= (1 << pos)
                    if mask not in seen:
                        seen.add(mask)
                        placements.append(mask)
        return placements

    placements_by_shape = {idx: placements_for_shape(idx) for idx in range(len(shapes))}

    placements_list = [placements_by_shape[idx] for idx in order]
    if any(not placements for placements in placements_list):
        return False

    order.sort(key=lambda si: (len(placements_by_shape[si]), -len(shapes[si]['variants'][0]['shape'])))
    placements_list = [placements_by_shape[idx] for idx in order]

    @cache
    def backtrack(idx, occupied_mask):
        if idx >= len(order):
            return True
        occ_count = occupied_mask.bit_count()
        remaining_area = sum(area_map[order[i]] for i in range(idx, len(order)))
        if remaining_area > w * h - occ_count:
            return False
        for placement in placements_list[idx]:
            if placement & occupied_mask:
                continue
            if backtrack(idx + 1, occupied_mask | placement):
                return True
        return False

    return backtrack(0, 0)

def part1():
    count = 0
    for region in enumerate(regions):
        if solve(region[1], shapes):
            count += 1
    return count

def part2():
    return 0

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
