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


matrix = [[1,1,1],[1,0,1],[1,1,1]]
zero_matrix(matrix)
assert matrix == [[1,0,1],[0,0,0],[1,0,1]]
