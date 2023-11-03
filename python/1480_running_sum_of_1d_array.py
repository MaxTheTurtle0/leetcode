def runningSum(nums: list[int]) -> list[int]:
    running_sum = []
    for i, _ in enumerate(nums):
        running_sum.append(sum(nums[0:i+1]))
    return running_sum


def runningSumSmart(nums: list[int]) -> list[int]:
    running_sum = []
    curr_sum = 0
    for v in nums:
        curr_sum += v
        running_sum.append(curr_sum)
    return running_sum

def runningSumSmart2(nums: list[int]) -> list[int]:
    for i in range(1, len(nums)):
        nums[i] += nums[i-1]
    return nums
