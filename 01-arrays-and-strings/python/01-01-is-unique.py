# w/ data structure (set)
def unique(s: str) -> bool:
    chars = set()
    for ch in s:
        chars.add(ch)
    return len(chars) == len(s)

assert unique("abcd"), unique("abcd")
assert not unique("dddd"), unique("dddd")

# w/out data structure (bitset)
def unique(s: str) -> bool:
    bits = 0
    for ch in s:
        # a - z
        letter_mask = 1 << ord(ch) - 97
        contains = bits & letter_mask
        if contains == letter_mask:
            return False
        bits |= letter_mask
    return True

assert unique("abcd"), unique("abcd")
assert not unique("dddd"), unique("dddd")
