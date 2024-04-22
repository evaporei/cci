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
def leet(s: list[str]) -> int:
    old_len = len(s)

    index = 0
    i = 0
    while i < len(s):
        j = i
        while j < len(s) and s[i] == s[j]:
            j += 1
        s[index] = s[i]
        index += 1
        count = j - i
        if count > 1:
            for ch in str(count):
                s[index] = ch
                index += 1
        i = j

    if old_len > index:
        del s[index:]
        return index

    return old_len


expected = {
    "aaaaabcccccdeeef": "a5bc5de3f",
    "aaabbbcccddd": "a3b3c3d3",
    "aabbbbcdddd": "a2b4cd4",
    "aabcccccaaa": "a2bc5a3",
    "abccddefg": None,
    "abcdefg": None
}

for input, output in expected.items():
    l_input = list(input)
    length = leet(l_input)
    if output:
        assert length == len(output)
        assert l_input == list(output)
    else:
        assert length == len(input)
