#!/usr/bin/python

import sys
import math
import itertools

def northof(board, i):
    # assume board is a square
    rowsize = math.sqrt(len(board))
    if i - rowsize >= 0:
        return int(i - rowsize)
    else: return -1

def southof(board, i):
    # assume board is a square
    rowsize = math.sqrt(len(board))
    if i + rowsize < len(board):
        return int(i + rowsize)
    else: return -1

def eastof(board, i):
    # assume board is a square
    rowsize = math.sqrt(len(board))

    # the modulus would only equal zero if it wrapped
    if i + 1 < len(board) and (i + 1) % rowsize != 0:
        return i + 1
    else: return -1

def westof(board, i):
    # assume board is a square
    rowsize = math.sqrt(len(board))

    # the modulus would only equal 5 if it wrapped
    if i - 1 >= 0 and (i - 1) % rowsize != 4:
        return i - 1
    else: return -1

def placetile(board, i):
    # print("---")
    # print(f"Placing tile at: {i}")
    board[i] = True
    points = 1
    rp = rowpoints(board, i)
    cp = colpoints(board, i)
    # print("Tilepoints: 1")
    if rp > 0 and cp > 0:
        points += 1
    points += rp + cp
    # printboard(board, points)
    return points
    
def rowpoints(board, i):
    points = eastpoints(board, i) + westpoints(board, i)
    # print(f"Row Neighbors: {points}")
    return points

def colpoints(board, i):
    points = northpoints(board, i) + southpoints(board, i)
    # print(f"Col Neighbors: {points}")
    return points

def northpoints(board, i):
    if northof(board, i) == -1 or board[northof(board, i)] == False:
        return 0
    else: return 1 + northpoints(board, northof(board, i))

def southpoints(board, i):
    if southof(board, i) == -1 or board[southof(board, i)] == False:
        return 0
    else: return 1 + southpoints(board, southof(board, i))

def eastpoints(board, i):
    if eastof(board, i) == -1 or board[eastof(board, i)] == False:
        return 0
    else: return 1 + eastpoints(board, eastof(board, i))

def westpoints(board, i):
    if westof(board, i) == -1 or board[westof(board, i)] == False:
        return 0
    else: return 1 + westpoints(board, westof(board, i))

def gtp_recursive_backtracking(board, startpos, size):
    visited = {}
    patterns = []
    pos = startpos
    path = []
    rb_takestep(board, visited, patterns, path, size, pos, pos)
    return patterns

def rb_takestep(board, visited, patterns, path, steps, pos, startpos):
    # don't step where already stepped
    path.append(pos)
    visited[pos] = True
    steps -= 1
    if steps == 0:
        patterns.append(path.copy())
        path.pop()
        del visited[pos]
        return
    
    neighbors = [northof(board, pos), southof(board, pos), eastof(board, pos), westof(board, pos)]
    for neighbor in neighbors:
        if neighbor != -1 and neighbor not in visited:
            rb_takestep(board, visited, patterns, path, steps, neighbor, startpos)
    path.pop()
    del visited[pos]
    
def gtp_puddle(board, startpos, size):
    frontier = {startpos}
    visited = set()
    patterns = {0: set()}

    for i in range(size):
        patterns[i+1] = explore(board, patterns[i], frontier).copy()
        print(f"iteration {i}: {patterns}")
    return patterns, frontier
    
    # frontier is all unexplored nodes adjacent to the current pattern
    # if the pattern is smaller than the desired size, explore a frontier

def explore(board, pattern, frontier):
    patterns = []
    for pos in frontier.copy():
        frontier.remove(pos)
        np = pattern.copy()
        np.add(pos)
        if np not in patterns:
            patterns.append(np)
            if northof(board, pos) not in np:
                frontier.add(northof(board, pos))
            if southof(board, pos) not in np:
                frontier.add(southof(board, pos))
            if eastof(board, pos) not in np:
                frontier.add(eastof(board, pos))
            if westof(board, pos) not in np:
                frontier.add(westof(board, pos))
    return patterns

def printboard(board, frontier, points):
    print("---")
    for i in range(5):
        for j in range(5):
            if board[5*i + j]:
                print(" O ", end='')
            elif frontier is not None and 5*i + j in frontier:
                print(" + ", end='')
            else:
                print(" . ", end='')
        print()
    print("---")
    # print(f"Points for this placement: {points}")

# 0  1  2  3  4
# 5  6  7  8  9
# 10 11 12 13 14
# 15 16 17 18 19
# 20 21 22 23 24

# .  .  .  .  .  
# .  .  +  .  .
# .  +  O  +  .
# .  .  +  .  .
# .  .  .  .  .

# use recursive backtracking algorithm to get tile locations
# then send those locations into a permutation generator to get
# all tile placement orders for each pattern

if len(sys.argv) < 3:
    helpmessage = '''USAGE: ./main.py <startpos> <tilecount>
Tile Positions
--------------
0  1  2  3  4
5  6  7  8  9
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24
--------------'''
    sys.exit(helpmessage)
board = [False for _ in range(25)]
# patterns = gtp_recursive_backtracking(board, int(sys.argv[1]), int(sys.argv[2]))
patterns, frontier = gtp_puddle(board, int(sys.argv[1]), int(sys.argv[2]))

# display patterns
bestpattern = (None, 0)
pointdistribution = {}
for pattern in patterns:
    permutations = list(itertools.permutations(pattern))
    # print(permutations)
    board = [False for _ in range(25)]
    bestpermutation = (None, 0)
    for p, permutation in enumerate(permutations):
        board = [False for _ in range(25)]
        points = 0
        for tile in permutation:
            points += placetile(board, tile)
        
        if points not in pointdistribution:
            pointdistribution[points] = 0
        pointdistribution[points] += 1
        if points > bestpermutation[1]:
            bestpermutation = (permutation, points)
        # print(f"Points for permutation {p}: {points}")
    if bestpermutation[1] > bestpattern [1]:
        bestpattern = bestpermutation
    print(f"Sample best permutation: {bestpermutation[0]}, Pointvalue: {bestpermutation[1]}")

    board = [False for _ in range(25)]
    for tile in bestpermutation[0]:
        placetile(board, tile)
        printboard(board, frontier, points)

board = [False for _ in range(25)]
print("==============")
print(" BEST PATTERN ")
print("==============")
print(f"Points: {bestpattern[1]}")
sortedpointdistribution = sorted(pointdistribution.items(), key=lambda x:x[1])
for k in sortedpointdistribution:
    print(f"Permutations worth {k[0]} points: {k[1]}")
for tile in bestpattern[0]:
    placetile(board, tile)
    printboard(board, frontier, points)