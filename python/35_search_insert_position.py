def searchInsert(nums: list[int], target: int) -> int:
    for i, _ in enumerate(nums):
        if nums[i] == target or nums[i] > target: return i
    return len(nums)
