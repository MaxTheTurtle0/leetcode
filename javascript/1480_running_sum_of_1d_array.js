/**
 * @param {number[]} nums
 * @return {number[]}
 */
function runningSum(nums) {
    for (let i = 0; i < nums.length; i++) {
        nums[i] += nums[i - 1];
    }
    return nums;
};
