# time: O(n.m)
# space: O(n+m)
def zero_matrix(m: list[list[int]]):
    rows = []
    cols = []
    for i in range(len(m)):
        for j in range(len(m[i])):
            if m[i][j] == 0:
                rows.append(i)
                cols.append(j)

    for i in rows:
        for j in range(len(m[i])):
            m[i][j] = 0

    for j in cols:
        for i in range(len(m)):
            m[i][j] = 0


# time: O(n.m)
# space: O(1)
def optimal(m: list[list[int]]):
    ROWS, COLS = len(m), len(m[0])
    row_zero = False
    for r in range(ROWS):
        for c in range(COLS):
            if m[r][c] == 0:
                m[0][c] = 0
                if r > 0:
                    m[r][0] = 0
                else:
                    row_zero = True

    for r in range(1, ROWS):
        for c in range(1, COLS):
            if m[r][0] == 0 or m[0][c] == 0:
                m[r][c] = 0

    if m[0][0] == 0:
        for r in range(ROWS):
            m[r][0] = 0

    if row_zero:
        for c in range(COLS):
            m[0][c] = 0


matrix = [[1,1,1],[1,0,1],[1,1,1]]
zero_matrix(matrix)
assert matrix == [[1,0,1],[0,0,0],[1,0,1]]

matrix = [[1,1,1],[1,0,1],[1,1,1]]
optimal(matrix)
assert matrix == [[1,0,1],[0,0,0],[1,0,1]]
