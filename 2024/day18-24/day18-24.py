
# read command-line parameters and based on that read the input file
from collections import defaultdict
import heapq
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day18-{runtype}.txt", "r")

lines = [list(line.strip().split(',')) for line in text_file.readlines()]
byte_locations = [[int(v) for v in pair] for pair in lines]
byte_locations = [(x, y) for x, y in byte_locations]
size = (7,7) if runtype == 'test' else (71,71)

def map_after_ns(time, byte_locations, size):
    byte_map = set()
    for x in range(size[0]):
        for y in range(size[1]):
            byte_map.add((x, y))
    for x, y in byte_locations[:time]:
        byte_map.remove((x, y))
    return byte_map

def find_path(byte_map, start, end):
    backtrack = {}
    dist = defaultdict(lambda: float('inf'))
    dist[start] = 0

    def gen_path():
        current = end
        while current != start:
            yield current
            current = backtrack[current]
        yield start


    q = []
    heapq.heappush(q, (0, start))
    seen = set()
    while q:
        cost, v1 = heapq.heappop(q)
        if v1 not in seen:
            seen.add(v1)
            if v1 == end:
                return gen_path()
            
            for dx, dy in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
                v2 = (v1[0] + dx, v1[1] + dy)
                if v2 in byte_map:
                    if dist[v1] + 1 < dist[v2]:
                        dist[v2] = dist[v1] + 1
                        backtrack[v2] = v1
                        heapq.heappush(q, (cost+1, v2))
    return None

def print_map(map, size):
    for y in range(size[1]):
        for x in range(size[0]):
            if (x, y) in map:
                print('.', end='')
            else:
                print('#', end='')
        print()

def part1():
    bytes_fallen = 12 if runtype == 'test' else 1024
    map = map_after_ns(bytes_fallen, byte_locations, size)
    start = (0,0)
    end = (size[0]-1, size[1]-1)
    path = find_path(map, start, end)
    if path:
        return len(list(path))-1

def part2():
    t = 1
    while True:
        t += 1
        map = map_after_ns(t, byte_locations, size)
        start = (0,0)
        end = (size[0]-1, size[1]-1)
        if not find_path(map, start, end):
            break
    return byte_locations[t-1]

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
