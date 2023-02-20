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
    if i - 1 >= 0 and (i - 1) % rowsize != 5:
        return i - 1
    else: return -1

def placetile(board, i):
    print("---")
    print(f"Placing tile at: {i}")
    board[i] = True
    points = 1
    rp = rowpoints(board, i)
    cp = colpoints(board, i)
    print("Tilepoints: 1")
    if rp > 0 and cp > 0:
        points += 1
    points += rp + cp
    printboard(board, points)
    
def rowpoints(board, i):
    points = eastpoints(board, i) + westpoints(board, i)
    print(f"Row Neighbors: {points}")
    return points

def colpoints(board, i):
    points = northpoints(board, i) + southpoints(board, i)
    print(f"Col Neighbors: {points}")
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

def generatetilepatterns(board, size):
    # this may be a dynamic programming problem
    return 0

def printboard(board, points):
    for i in range(5):
        for j in range(5):
            print(f" {board[5*i + j]}", end='')
        print()
    print(f"Points: {points}")

# 0  1  2  3  4
# 5  6  7  8  9
# 10 11 12 13 14
# 15 16 17 18 19
# 20 21 22 23 24

board = [False for _ in range(25)]
# print(board)
# points = placetile(board, 12)
printboard(board, 0)
placetile(board, 12)
placetile(board, 8)
placetile(board, 7)
placetile(board, 22)
placetile(board, 17)

print(list(itertools.permutations([1, 2, 3])))
