# b -> b1
def compress(s: str) -> str:
    out = []
    letter = s[0]
    count = 0
    for ch in s:
        if ch == letter:
            count += 1
        else:
            out.append(letter)
            out.append(str(count))
            letter = ch
            count = 1

    out.append(letter)
    out.append(str(count))

    if len(out) >= len(s):
        return s
    return "".join(out)


res = compress("aabcccccaaa")
assert res == "a2b1c5a3", res
assert compress("aaaaabcccccdeeef") == "a5b1c5d1e3f1"
assert compress("aaabbbcccddd") == "a3b3c3d3"
assert compress("aabbbbcdddd") == "a2b4c1d4"
assert compress("aabcccccaaa") == "a2b1c5a3"
assert compress("abccddefg") == "abccddefg"
assert compress("abcdefg") == "abcdefg"


# b -> b
def leet(s: str) -> str:
    out = []
    letter = s[0]
    count = 0
    for ch in s:
        if ch == letter:
            count += 1
        else:
            out.append(letter)
            if count != 1:
                out.append(str(count))
            letter = ch
            count = 1

    out.append(letter)
    if count != 1:
        out.append(str(count))

    if len(out) >= len(s):
        return s
    return "".join(out)


assert leet("aaaaabcccccdeeef") == "a5bc5de3f"
assert leet("aaabbbcccddd") == "a3b3c3d3"
assert leet("aabbbbcdddd") == "a2b4cd4"
assert leet("aabcccccaaa") == "a2bc5a3"
assert leet("abccddefg") == "abccddefg"
assert leet("abcdefg") == "abcdefg"
