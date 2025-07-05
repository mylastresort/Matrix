from sympy import Matrix, I


def stats(m):
    # Compute the reduced row echelon form (RREF)
    rref_matrix, pivot_columns = m.rref()

    # Print the RREF matrix
    print("Reduced Row Echelon Form (RREF):")
    print(rref_matrix)

    # Print pivot columns
    print("Pivot columns:", pivot_columns)

    # Print the rank of the matrix
    print("Rank of the matrix:", m.rank())

    # Inverse of the matrix
    print("Inverse of the matrix", m.inv())


# Real matrices (imaginary part zero)
u1 = Matrix([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])

u2 = Matrix([[1.0, 2.0, 0.0, 0.0], [2.0, 4.0, 0.0, 0.0], [-1.0, 2.0, 1.0, 1.0]])


u3 = Matrix([[8.0, 5.0, -2.0], [4.0, 7.0, 20.0], [7.0, 6.0, 1.0], [21.0, 18.0, 7.0]])
stats(u3)

u4 = Matrix(
    [
        [1 + 12 * I, 2 + 13 * I, 3 + 14 * I],
        [4 + 15 * I, 5 + 16 * I, 6 + 17 * I],
        [7 + 18 * I, 8 + 19 * I, 9 + 20 * I],
    ]
)

u5 = Matrix(
    [
        [1 + 12 * I, 2 + 13 * I, 3 + 14 * I, 4 + 15 * I],
        [5 + 16 * I, 6 + 17 * I, 7 + 18 * I, 8 + 19 * I],
        [9 + 20 * I, 10 + 21 * I, 11 + 22 * I, 12 + 23 * I],
    ]
)

u6 = Matrix(
    [
        [41 + 12 * I, 22 + 13 * I, 13 + 14 * I, 4 + 15 * I, 5 + 16 * I],
        [6 + 17 * I, 7 + 18 * I, 8 + 19 * I, 9 + 20 * I, 10 + 21 * I],
        [11 + 22 * I, 12 + 23 * I, 14 + 24 * I, 15 + 25 * I, 16 + 26 * I],
        [17 + 27 * I, 18 + 28 * I, 19 + 29 * I, 20 + 30 * I, 21 + 31 * I],
        [22 + 32 * I, 23 + 33 * I, 24 + 34 * I, 25 + 35 * I, 26 + 36 * I],
    ]
)

u7 = Matrix(
    [
        [1 + 12 * I, 2 + 13 * I, 3 + 14 * I, 4 + 15 * I],
        [5 + 16 * I, 6 + 17 * I, 7 + 18 * I, 8 + 19 * I],
        [9 + 20 * I, 10 + 21 * I, 11 + 22 * I, 12 + 23 * I],
        [13 + 24 * I, 14 + 25 * I, 15 + 26 * I, 16 + 27 * I],
        [17 + 28 * I, 18 + 29 * I, 19 + 30 * I, 20 + 31 * I],
    ]
)

# stats(u7)