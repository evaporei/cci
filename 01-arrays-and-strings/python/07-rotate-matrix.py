# 90 degrees
def rotate(m: list[list[int]]):
    for first in range(int(len(m) / 2)):
        last = len(m) - first - 1
        for i in range(first, last):
            offset = i - first
            # save
            top = m[first][i]
            # left -> top
            m[first][i] = m[last - offset][first]
            # bottom -> left
            m[last - offset][first] = m[last][last - offset]
            # right -> bottom
            m[last][last - offset] = m[i][last]
            # top -> right
            m[i][last] = top


def neet(m: list[list[int]]):
    left, right = 0, len(matrix) - 1

    while left < right:
        for i in range(right - left):
            top, bottom = left, right

            # save
            top_left = m[top][left + i]
            
            m[top][left + i] = m[bottom - i][left]

            m[bottom - i][left] = m[bottom][right - i]
            
            m[bottom][right - i] = m[top + i][right]

            m[top + i][right] = top_left

        left += 1
        right -= 1



# 1 2 3      7 4 1
# 4 5 6  ->  8 5 2
# 7 8 9      9 6 3
matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
rotate(matrix)
assert matrix == [[7, 4, 1], [8, 5, 2], [9, 6, 3]]

matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
neet(matrix)
assert matrix == [[7, 4, 1], [8, 5, 2], [9, 6, 3]]
