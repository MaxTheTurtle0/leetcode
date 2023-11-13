/**
 * @param {number[]} nums
 * @return {number}
 */
function singleNumber(nums) {
    let answer = 0;
    for (let i = 0; i < nums.length; i++) answer ^= nums[i];
    return answer;
};
