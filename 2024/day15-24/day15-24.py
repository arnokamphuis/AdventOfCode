
# read command-line parameters and based on that read the input file
from collections import defaultdict
from copy import deepcopy
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day15-{runtype}.txt", "r")

lines = [line.strip() for line in text_file.readlines()]
emptyline = lines.index('')
operations = ''.join(lines[emptyline+1:])

size = (len(lines[0]), emptyline)
map = defaultdict(int)
objects = []
start = None

for y in range(size[1]):
    for x in range(size[0]):
        if lines[y][x] != '#':
            map[(x, y)] = 1
        if lines[y][x] == '@':
            start = (x, y)
        if lines[y][x] == 'O':
            objects.append([(x, y)])
            

def get_all_object_positions(objects, objects_to_do = None):
    objs = []
    for obj in objects:
        if objects_to_do is None or obj in objects_to_do:
            objs.extend(get_object_positions(obj))
    return objs
                
def get_object_positions(obj):
    return obj
    # if len(obj) == 1:
    #     objs = [obj[0]]
    # else:
    #     objs = [p for ob in obj for p in ob]
    # return objs
    
def print_map(map, objects, pos, size):
    objs = get_all_object_positions(objects)
    if len(objects[0]) == 1:
        objs = [obj[0] for obj in objects]
    else:
        objs = [p for obj in objects for p in obj]
    for y in range(size[1]):
        for x in range(size[0]):
            if (x, y) == pos:
                print('@', end='')
            elif (x, y) in objs:
                print('O', end='')
            elif (x,y) in map:
                print('.', end='')
            else:
                print('#', end='')
        print()
    print()    
    
def get_index_of_object(objects, pos):
    for i, obj in enumerate(objects):
        if pos in obj:
            return i
    return -1
    
    
def get_objects_at_position(objects, pos_set):
    return [obj for obj in objects if any([p in obj for p in pos_set])]

def get_object_ids_at_position(objects, pos_set):
    ids = []
    for i, obj in enumerate(objects):
        if any([p in obj for p in pos_set]):
            ids.append(i)
    return ids
    
def can_move(map, objects, pos_set, dir):
    next_pos_set = set([(pos[0] + dir[0], pos[1] + dir[1]) for pos in pos_set])
    
    obj_ids_in_front = set(get_object_ids_at_position(objects, next_pos_set))
    
    object_pos_in_front = set()
    for obj_id in obj_ids_in_front:
        object_pos_in_front = object_pos_in_front.union(set(objects[obj_id]))
    
    next_object_pos = set([(pos[0] + dir[0], pos[1] + dir[1]) for pos in object_pos_in_front])
    object_pos_in_front = next_object_pos.difference(object_pos_in_front)
    object_pos_in_front = set([(pos[0]-dir[0], pos[1]-dir[1]) for pos in object_pos_in_front])
    
    free_pos_in_front = next_pos_set.difference(object_pos_in_front)
    if all([pos in map for pos in free_pos_in_front]): # no walls in the way
        if len(obj_ids_in_front) == 0: # no objects in the way
            return True, set()
        
        result = can_move(map, objects, object_pos_in_front, dir)
        if result[0]:
            return True, obj_ids_in_front.union(result[1])
    
    return False, set()

def do_operations(map, objects, start, operations, size):
    op_index = 0
    robot = start
    dirs = {'^': (0, -1), '>': (1, 0), 'v': (0, 1), '<': (-1, 0)}    
    while op_index < len(operations):
        op = dirs[operations[op_index]]

        can, obj_ids = can_move(map, objects, set([robot]), op)
        if can:
            for index in obj_ids:
                for pi in range(len(objects[index])):
                    objects[index][pi] = (objects[index][pi][0] + op[0], objects[index][pi][1] + op[1])
            robot = (robot[0] + op[0], robot[1] + op[1])
        op_index += 1
    
    return sum([  100*obj[0][1] + obj[0][0] for obj in objects])

def part1():
    return do_operations(deepcopy(map), deepcopy(objects), start, operations, size)

def part2():
    widemap = defaultdict(int)
    for (x, y) in map.keys():
        widemap[(2*x+0, y)] = 1
        widemap[(2*x+1, y)] = 1
    wide_objects = deepcopy(objects)
    for i in range(len(wide_objects)):
        wide_objects[i] = [(2*wide_objects[i][0][0]+0, wide_objects[i][0][1]),
                           (2*wide_objects[i][0][0]+1, wide_objects[i][0][1])
                           ]
    wide_start = (2*start[0], start[1])
    wide_size = (2*size[0], size[1])
    
    # print_map(widemap, wide_objects, wide_start, wide_size)
    return do_operations(widemap, wide_objects, wide_start, operations, wide_size)

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
