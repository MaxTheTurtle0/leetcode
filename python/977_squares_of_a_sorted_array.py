def sortedSquares(nums: list[int]) -> list[int]:
    return sorted([num*num for num in nums])

def sortedSquaresFast(nums: list[int]) -> list[int]:
    n = len(nums)
    vec = [0] * n

    start = 0
    end = n - 1

    for idx in range(n - 1, -1, -1):
        if abs(nums[end]) > abs(nums[start]):
            vec[idx] = nums[end] * nums[end]
            end -= 1 
        else:
            vec[idx] = nums[start] * nums[start]
            start += 1
    return vec
