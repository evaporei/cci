# def urlfy(s: str) -> str:
#     return s.replace(" ", "%20")

# def urlfy(s: str) -> str:
#     ret = ""
#     for ch in s:
#         if ch == " ":
#             ret += "%20"
#         else:
#             ret += ch
#
#     return ret

# assert urlfy("MrJohnSmith") == "MrJohnSmith"
# assert urlfy("Mr John Smith") == "Mr%20John%20Smith"

def urlfy(s: list[str]) -> list[str]:
    for i in range(len(s)):
        if s[i] == " ":
            s[i] = "%"
            s[i+3:] = s[i+1:-2]
            s[i+1] = "2"
            s[i+2] = "0"

    return s


assert urlfy(list("MrJohnSmith")) == list("MrJohnSmith")
res = urlfy(list("Mr John Smith") + [""] * 4)
assert res == list("Mr%20John%20Smith"), res
