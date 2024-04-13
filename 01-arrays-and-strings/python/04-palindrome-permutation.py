def palindrome_permutation(s: str) -> bool:
    count = {}
    for ch in s:
        if not ch.isalpha():
            continue
        ch = ch.lower()
        if ch in count:
            count[ch] += 1
        else:
            count[ch] = 1
    odds = [v for v in count.values() if v % 2 == 1]
    return len(odds) <= 1


def palindrome_bits(s: str) -> bool:
    bitset = 0
    for ch in s:
        if not ch.isalpha():
            continue
        index = ord(ch.lower()) - ord('a')
        mask = 1 << index
        bitset ^= mask
    return bitset == 0 or bitset & (bitset - 1) == 0


assert palindrome_permutation("Tact Coa")
assert not palindrome_permutation("abcd")
assert palindrome_permutation("akkak")

assert palindrome_bits("Tact Coa")
assert not palindrome_bits("abcd")
assert palindrome_bits("akkak")
