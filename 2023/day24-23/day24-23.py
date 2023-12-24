
# read command-line parameters and based on that read the input file
import sys
import sympy
runtype = sys.argv[1]
runpart = int(sys.argv[2])
if len(sys.argv) > 3:
    runs = int(sys.argv[3])
else:
    runs = 1

text_file = open(f"day24-{runtype}.txt", "r")

trajectories = [[list(map(int, [n for n in part.split(', ')])) for part in line.split(' @ ')] for line in text_file.readlines()]

# for i, t in enumerate(trajectories):
#     print(i, t[0], t[1])

test_area_x = [200000000000000, 400000000000000] if runtype == 'real' else [7, 27]
test_area_y = [200000000000000, 400000000000000] if runtype == 'real' else [7, 27]

def path_intersect(t1, t2):
    # write everything in the form ax + by = c
    a1 = t1[1][1]
    b1 = -t1[1][0]
    c1 = a1 * t1[0][0] + b1 * t1[0][1]
    a2 = t2[1][1]
    b2 = -t2[1][0]
    c2 = a2 * t2[0][0] + b2 * t2[0][1]

    # if both directional coefficients are equal, the lines are parallel
    # v1y * -v2x == -v1x * v2y  ===>  v1x * v2y - v1y * v2x == 0 (cross product is 0)
    if a1 * b2 == b1 * a2:
        return False, None
    
    # otherwise, the lines intersect at a single point
    point = [ 
        (c1 * b2 - c2 * b1) / (a1 * b2 - a2 * b1),
        (c2 * a1 - c1 * a2) / (a1 * b2 - a2 * b1)
    ]

    # check when the point is on the line segments
    tx_1 = (point[0] - t1[0][0]) / t1[1][0]
    ty_1 = (point[1] - t1[0][1]) / t1[1][1]
    tx_2 = (point[0] - t2[0][0]) / t2[1][0]
    ty_2 = (point[1] - t2[0][1]) / t2[1][1]

    # time is always positive, so if it's negative, the point is not on the line segment
    return tx_1 >= 0 and ty_1 >= 0 and tx_2 >= 0 and ty_2 >= 0, point

def part1():
    res = 0
    for i1, t1 in enumerate(trajectories):
        for t2 in trajectories[:i1]:
            intersect, position = path_intersect(t1, t2)
            if intersect and test_area_x[0] <= position[0] <= test_area_x[1] and test_area_y[0] <= position[1] <= test_area_y[1]:
                res += 1
    return res

def part2():
    # we need to solve for intersections like in part 1.
    # however, we need to solve this for every trajectory with respect to the unknown trajectory
    # the unknown trajectory is defined by the following variables:
    xr, yr, zr, vxr, vyr, vzr = sympy.symbols("xr, yr, zr, vxr, vyr, vzr")

    # we need to solve this system of equations for every trajectory
    # this requires 2n equations, where n is the number of trajectories
    # the equations are the determinant of the matrix
    # https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection

    equations = []
    for i, t in enumerate(trajectories):
        equations.append( (xr - t[0][0]) * (t[1][1] - vyr) - (yr - t[0][1]) * (t[1][0] - vxr) )
        equations.append( (xr - t[0][0]) * (t[1][2] - vzr) - (zr - t[0][2]) * (t[1][0] - vxr) )
    (sol,) = sympy.solve(equations, [xr, yr, zr, vxr, vyr, vzr])
    return sum(sol[:3])

if runpart == 1 or runpart == 0:
    for run in range(runs):
        resp1 = part1()
    print("Part 1: {}".format(resp1))

if runpart == 2 or runpart == 0:
    for run in range(runs):
        resp2 = part2()
    print("Part 2: {}".format(resp2))
    
