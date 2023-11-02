/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
function searchInsertLinear(nums, target) {
    for (let i = 0; i < nums.length; i++) {
        if (nums[i] >= target) return i;
    }
    return nums.length
};

/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
function searchInsertBinary(nums, target) {
    let left = 0;
    let right = nums.length - 1;
    let middle = Math.round((left + right) / 2);
    while (left <= right) {
        if (nums[middle] === target) return middle;
        else if (nums[middle] > target) right = middle - 1;
        else if (nums[middle] < target) left = middle + 1;
        middle = Math.round((left + right) / 2);
    }
    return middle;
};
