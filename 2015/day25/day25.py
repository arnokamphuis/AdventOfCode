

def next(prev):
    return (prev * 252533) % 33554393

n = 20151125


def nextcoor(c):
    ncr = c[0]-1
    ncc = c[1]+1
    if ncr < 1:
        ncr = ncc
        ncc = 1
    return (ncr, ncc)

count = 1
coor = (1,1)

# row 2947, column 3029.

target = (2947, 3029)

while coor != target:
    n = next(n)  
    coor = nextcoor(coor)
    # print(n, coor)
print(n)
