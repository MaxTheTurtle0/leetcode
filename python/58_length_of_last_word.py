def lengthOfLastWord(s: str) -> int:
    s = s.split(" ")
    for word in s[::-1]:
        if word != "":
            return len(word)
