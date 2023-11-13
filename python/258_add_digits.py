def addDigits(num: int) -> int:
    num = sum([int(digit) for digit in str(num)])
    if len(str(num)) > 1: num = addDigits(num)
    return num

def addDigitsSmart(num: int) -> int:
    while num > 9: num = num % 10 + num // 10
    return num
