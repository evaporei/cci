def one_away(s: str, t: str) -> bool:
    if len(s) == len(t):
        return replace(s, t)
    elif len(s) > len(t):
        return remove(s, t)
    else:
        return remove(t, s)


def replace(s: str, t: str) -> bool:
    one = False
    for c1, c2 in zip(s, t):
        if c1 != c2:
            if one:
                return False
            else:
                one = True
    return one


def remove(s: str, t: str) -> bool:
    one = False
    i = 0
    j = 0
    while j < len(t):
        if s[i] != t[j]:
            if one:
                return False
            i += 1
            one = True
        i += 1
        j += 1
    return one or i != len(s)


assert not one_away("pale", "bake")
# replace
assert one_away("pale", "bale")
# remove
assert one_away("pale", "ple")
# insert
assert one_away("pales", "pale")
