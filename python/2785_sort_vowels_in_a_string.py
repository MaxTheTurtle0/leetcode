def sortVowelsSlow(s: str) -> str:
    s_list = list(s)
    vowels = 'aeiouAEIOU'
    for i in range(len(s) - 1):
        for j in range(i + 1, len(s)):
            if s_list[j] in vowels and s_list[i] in vowels:
                if ord(s_list[j]) < ord(s_list[i]):
                    s_list[i], s_list[j] = s_list[j], s_list[i]
    return "".join(s_list)


def sortVowels(s: str) -> str:
    sorted_vowels = sorted([c for c in s if c.lower() in "aeiou"], reverse=True)

    s_list = []
    for c in s:
        if c.lower() in "aeiou":
            s_list.append(sorted_vowels.pop())
        else:
            s_list.append(c)
    return "".join(s_list)
