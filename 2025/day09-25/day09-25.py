
# read command-line parameters and based on that read the input file
from copy import deepcopy
import sys
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day09-{runtype}.txt", "r")

map = [tuple(map(int, line.strip().split(','))) for line in text_file.readlines()]

def part1():
    max_area = None
    for p1 in map:
        for p2 in map:
            if p1 != p2:
                size = (min(p1[0], p2[0]), min(p1[1], p2[1])), (max(p1[0], p2[0]), max(p1[1], p2[1]))
                area = (size[1][0] - size[0][0] + 1) * (size[1][1] - size[0][1] + 1)
                if max_area is None or area > max_area:
                    max_area = area
    return max_area

def strict_inside(p, rect):
    return rect[0][0] < p[0] < rect[2][0] and rect[0][1] < p[1] < rect[2][1]

def strict_crossing(edge, rect):
    # edge is (p1, p2)
    # rect is [p1, p3, p2, p4]

    rect_edges = [ (rect[0], rect[1]),
                   (rect[1], rect[2]),
                   (rect[2], rect[3]),
                   (rect[3], rect[0]) ]
    
    x_min, x_max = min(rect[0][0], rect[2][0]), max(rect[0][0], rect[2][0])
    y_min, y_max = min(rect[0][1], rect[2][1]), max(rect[0][1], rect[2][1])
                        
    if edge[0][0] == edge[1][0]:
        # vertical edge
        x = edge[0][0]
        if x_min < x < x_max:
            y1 = min(edge[0][1], edge[1][1])
            y2 = max(edge[0][1], edge[1][1])

            if max(y_min, y1) < min(y_max, y2):
                return True
    else:
        # horizontal edge
        y = edge[0][1]
        if y_min < y < y_max:
            x1 = min(edge[0][0], edge[1][0])
            x2 = max(edge[0][0], edge[1][0])
            if max(x_min, x1) < min(x_max, x2):
                return True
    return False

def point_inside_polygon(point, edges):
    x, y = point
    intersections = 0
    for edge in edges:
        p1, p2 = edge
        x1, y1 = p1
        x2, y2 = p2
        
        # Check if edge is vertical and intersects the ray's y-level
        # We use strict inequality for one end to avoid double counting vertices
        if min(y1, y2) < y <= max(y1, y2) and x < x1:
            # For a vertical edge, x1 == x2. 
            # If the point is to the left of this vertical line, it intersects.
            intersections += 1
    return (intersections % 2) == 1

def part2():

    vertices = deepcopy(map)
    edges = [ (vertices[i],vertices[(i+1)%len(vertices)]) for i in range(len(vertices)) ]

    max_area = 0
    for i, p1 in enumerate(vertices):
        for j, p2 in enumerate(vertices):
            # print('-'*40)
            if i <= j:
                continue

            area = (abs(p2[0] - p1[0])+1) * (abs(p2[1] - p1[1])+1)
            if area <= max_area: continue

            center = ((p1[0] + p2[0]) / 2, (p1[1] + p2[1]) / 2)
            if not point_inside_polygon(center, edges): continue

            p3 = (p1[0], p2[1])
            p4 = (p2[0], p1[1])
            rect = [p1, p3, p2, p4]

            if not any([strict_crossing(edge, rect) for edge in edges]):
                max_area = area


    return max_area

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
