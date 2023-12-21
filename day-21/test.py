import time,sys
tstart=time.time()

inf = sys.argv[1] if len(sys.argv) > 1 else '21.in'
print('Gardens')
with open(inf) as fi:
    raw = [[y for y in x] for x in fi.read().strip().split('\n')]
    R=set()
    for r,l  in enumerate(raw): 
        for c,C in enumerate(l):
            if C == '#':
                R.add((r,c))
            if C == 'S':
                start = (r,c)
H,W = len(raw),len(raw[0])
dc,dr = [1,0,-1,0],[0,1,0,-1]
possible = {start}
points ={}
steps = 26501365
possibleLen = 0
for s in range(1,steps):
    new_possible = set()
    first_pattern = [set() for _ in range(9)]
    for r,c in possible:
        for i in range(4):
            rr,cc = r+dr[i],c+dc[i]
            #if 0<= rr <=H and 0<=cc<=H and (rr%H,cc%W) not in R:
            if (rr%H,cc%W) not in R:
                new_possible.add((rr,cc))
    possible = new_possible
    if s==64:
        print('part1',len(possible), 'after 64 steps')
        print('calculatin points')
    if s % H == steps % H:
        points[s//H] = len(possible)
        print('point',len(points), len(possible), 'after '+str(s)+' steps')
    if len(points) ==3:
        break
def f(n):
    y0 = points[0]
    y1 = points[1]
    y2 = points[2]
    a = (y2+y0-2*y1)/2
    b = y1-y0 -a
    c = y0
    return a*n**2 + b*n +c
print('part2',f(steps//H))

tend=time.time()
print(round(tend-tstart,5))