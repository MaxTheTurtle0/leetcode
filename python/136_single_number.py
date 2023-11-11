def singleNumberSlow(nums: list[int]) -> int:
    nums_new = []
    for num in nums:
        if num not in nums_new:
            nums_new.append(num)
        else:
            nums_new.remove(num)
    return int(nums_new[0])


def singleNumber(nums: list[int]) -> int:
    nums_dict = {}
    for num in nums:
        if num not in nums_dict.keys():
            nums_dict[num] = num
        else:
            nums_dict.pop(num)
    return int(list(nums_dict.values())[0])
