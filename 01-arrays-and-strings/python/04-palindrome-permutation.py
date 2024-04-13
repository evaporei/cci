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
    print(count)
    even = 0
    odd = 0
    for v in count.values():
        if v % 2 == 0:
            even += 1
        else:
            odd += 1
    return even == len(count) or (odd == 1 and even == len(count) - 1)

assert palindrome_permutation("Tact Coa")
assert not palindrome_permutation("abcd")
assert palindrome_permutation("akkak")
