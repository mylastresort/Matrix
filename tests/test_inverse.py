from sympy import Matrix, I, re, im


def format_entry(entry):
    # Format rational numbers as numerator/denominator
    if entry.is_Rational:
        return f"{entry.p}/{entry.q}"
    # For complex numbers, format real and imaginary parts separately
    elif entry.is_complex:
        real_part = re(entry)
        imag_part = im(entry)
        real_str = (
            f"{real_part.p}/{real_part.q}" if real_part.is_Rational else str(real_part)
        )
        imag_str = (
            f"{imag_part.p}/{imag_part.q}" if imag_part.is_Rational else str(imag_part)
        )
        sign = "+" if imag_part >= 0 else "-"
        return f"{real_str} {sign} {imag_str}*i"
    else:
        return str(entry)


def stats(m):
    # Inverse of the matrix
    print("Inverse of the matrix")
    print(m.inv())
    # print(m.inv(method="GE").applyfunc(format_entry))


# Real matrices (imaginary part zero)
u1 = Matrix([[1, 0, 0], [0, 1, 0], [0, 0, 1]])

# stats(u1)

u2 = Matrix([[2, 0, 0], [0, 2, 0], [0, 0, 2]])

# stats(u2)

u3 = Matrix([[8, 5, -2], [4, 7, 20], [7, 6, 1]])

# stats(u3)

# complex numbers matrix
u4 = Matrix([[1 + -1 * I, 2 + 3 * I], [4 + 5 * I, 6 + 7 * I]])

# u5 = Matrix([[C!(1., 0.), C!(2., 0.)], [C!(3., 0.), C!(4., 0.)]])

u5 = Matrix([[1, 2], [3, 4]])

# u6 = Matrix([[C!(1., 1.), C!(2., 2.)], [C!(3., 3.), C!(4., 4.)]])

u6 = Matrix([[1 + I, 2 + 2 * I], [3 + 3 * I, 4 + 4 * I]])

# u7 = Matrix([[C!(1., 2.), C!(3., 4.)], [C!(5., 6.), C!(7., 8.)]])

u7 = Matrix([[1 + 2 * I, 3 + 4 * I], [5 + 6 * I, 7 + 8 * I]])


print("\n\n")
stats(u4)
print("\n\n")
stats(u5)
print("\n\n")
stats(u6)
print("\n\n")
stats(u7)
