class Solution:
    def numIdenticalPairs(self, nums: list[int]) -> int:
        nums_len:int = len(nums)
        pairs:int = 0

        for i in range(0, nums_len):
            for j in range(i + 1, nums_len):
                if nums[i] == nums[j]: pairs += 1
        return pairs


