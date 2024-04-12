def is_permutation(a: str, b: str) -> bool:
    s = sorted(a)
    t = sorted(b)
    return s == t

assert is_permutation("abc", "cba")
assert not is_permutation("abc", "xyz")
