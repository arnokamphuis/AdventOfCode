# read command-line parameters and based on that read the input file
from collections import defaultdict
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day21-{runtype}.txt", "r")

lines = [list(line.strip()) for line in text_file.readlines()]

num_keypad = { '7': (0,0), '8': (1,0), '9': (2,0), \
               '4': (0,1), '5': (1,1), '6': (2,1), \
               '1': (0,2), '2': (1,2), '3': (2,2), \
               '0': (1,3), 'A': (2,3) }

dir_keypad = { '^': (1,0), 'A': (2,0), '<': (0,1), 'v': (1,1), '>': (2,1) }

dir_map = { (-1,0): '<', (1,0): '>', (0,-1): '^', (0,1): 'v' }

def find_pad_moves(from_key, to_key, pad):
    back_track = {}
    distances = defaultdict(lambda: float('inf'))
    visited = set()

    def path_to_key(pos):
        prev = back_track[pos]
        if len(prev) == 1:
            return [(prev[0], dir_map[(pos[0]-prev[0][0], pos[1]-prev[0][1])])]
        else:
            return [(back, dir_map[(pos[0]-back[0], pos[1]-back[1])]) for back in prev]

    def paths_to_key(pos):
        paths = [[(pos,' ')]]
        done = False
        while not done:
            new_paths = []
            for path in paths:
                lpe = path[-1]
                last_pos, _ = lpe
                ptks = path_to_key(last_pos)
                for ptk in ptks:
                    new_paths.append([*path,ptk])
            paths = new_paths
            done = all([path[-1][0]==from_key for path in paths])
        paths = [path[::-1] for path in paths]
        return paths
                        
    queue = [from_key]
    distances[from_key] = 0
    
    while queue:
        key = queue.pop(0)
        if key == to_key:
            return paths_to_key(key)
        if key in visited:
            continue
        visited.add(key)
        for dx, dy in dir_map.keys():
            new_key = (key[0]+dx, key[1]+dy)
            if new_key in pad.values() and new_key not in visited:
                if distances[new_key] > distances[key]+1: # if new key is closer
                    distances[new_key] = distances[key]+1
                    back_track[new_key] = [key]
                elif distances[new_key] == distances[key]+1:
                    back_track[new_key].append(key)
                else:
                    continue
                queue.append(new_key)
                
    return None

DP = {}
def find_directions(code, pad, current_location):
    key = ''.join(code)
    if key in DP:
        return DP[key]
    
    code = [current_location, *code]
    directions = []
    for i in range(1, len(code)):
        if code[i-1] != code[i]:
            pad_moves = find_pad_moves(pad[code[i-1]], pad[code[i]], pad)
            pad_moves = [[move for (_,move) in moves if move != ' '] for moves in pad_moves]
            pad_moves = [ [*moves, 'A'] for moves in pad_moves]
        else:
            pad_moves = [['A']]
        directions += [pad_moves]
    DP[key] = directions
    return directions

def gen_all_moves(code, pad, current_location):
    directions = find_directions(code, pad, current_location)
    all_moves = []
    
    for step in directions:
        new_all_moves = []
        for option in step:
            if all_moves == []:
                new_all_moves.append(option)
            for move in all_moves:
                new_all_moves.append([*move, *option])
        all_moves = new_all_moves
    return all_moves

def determine_pad(code):
    if all([c in num_keypad.keys() for c in code]):
        return num_keypad
    else:
        return dir_keypad

def generate(code, current_location):
    pad = determine_pad([*code,current_location])
    all_moves = gen_all_moves(code, pad, current_location)
    shortest = min([len(m) for m in all_moves])
    all_moves = [m for m in all_moves if len(m) == shortest]
    
    min_changes = min([sum([move[i] != move[i+1] for i in range(len(move)-1)]) for move in all_moves])
    all_moves = [move for move in all_moves if sum([move[i] != move[i+1] for i in range(len(move)-1)]) == min_changes]
    
    return all_moves

def gen_recursive(code, depth):
    if depth == 0:
        return code
    
    pad = determine_pad(code)
    res = []

    all_moves = gen_all_moves(code, pad)
    
    shortest = min([len(m) for m in all_moves])
    all_moves = [m for m in all_moves if len(m) == shortest]
    
    for move in all_moves:
        gen = gen_recursive(move, depth-1)
        res += [gen]
    
    return res

def split_full_code(code):
    split = []
    while 'A' in code:
        a_index = code.index('A')
        split.append([*code[:a_index], 'A'])
        code = code[a_index+1:]
    split = [''.join(s) for s in split]
    
    return split

cache = {}
def count_moves(code, current_location, depth):
    current_moves = [current_location, *code]
    # print(' '*(5-depth), "COUNT MOVES: ", current_moves, depth)
    key = (''.join(current_moves), depth)
    if key in cache:
        return cache[key]
    
    count = 0

    if depth == 0:
        count = len(code)
    else:
        all_moves = generate(code, current_location)
        min_count = float('inf')
        for moves in all_moves:
            c = 0
            split = split_full_code(moves)
            for s in split:
                c += count_moves(s, current_location, depth-1)
            if c < min_count:
                min_count = c       
        count += min_count
        
    # print(key, count)
    cache[key] = count
    return count


def part1():
    count = 0
    for code in lines:
        complexity = int(''.join(code[:-1]))
        c = count_moves(code, 'A', 3)
        count += c * complexity
    return count

def part2():
    count = 0
    for code in lines:
        complexity = int(''.join(code[:-1]))
        c = count_moves(code, 'A', 26)
        count += c * complexity
    return count

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
