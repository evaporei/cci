def is_rotation(s: str, goal: str) -> bool:
    return len(s) == len(goal) and s in goal + goal

assert is_rotation("waterbottle", "erbottlewat")
assert not is_rotation("waterbottle", "erbottlewater")
assert not is_rotation("abcde", "abced")
