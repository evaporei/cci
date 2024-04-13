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

# def urlfy(s: list[str]) -> list[str]:
#     for i in range(len(s)):
#         if s[i] == " ":
#             s[i] = "%"
#             s[i+3:] = s[i+1:-2]
#             s[i+1] = "2"
#             s[i+2] = "0"
#
#     return s

# assert urlfy(list("MrJohnSmith")) == list("MrJohnSmith")
# res = urlfy(list("Mr John Smith") + [""] * 4)
# assert res == list("Mr%20John%20Smith"), res

def urlfy(s: list[str], true_len: int):
    index = len(s) - 1
    i = true_len - 1
    while i >= 0:
        if s[i] == ' ':
            s[index] = '0'
            s[index - 1] = '2'
            s[index - 2] = '%'
            index -= 3
        else:
            s[index] = s[i]
            index -= 1
        i -= 1


wout_spaces = list("MrJohnSmith")
urlfy(wout_spaces, 11)
assert wout_spaces == list("MrJohnSmith"), wout_spaces

w_spaces = list("Mr John Smith") + [""] * 4
urlfy(w_spaces, 13)
assert w_spaces == list("Mr%20John%20Smith"), w_spaces
