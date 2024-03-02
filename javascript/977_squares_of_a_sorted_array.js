/**
 * @param {number[]} nums
 * @return {number[]}
 */
function sortedSquares(nums) {
    return nums.map((num) => num * num).sort();
};

/**
 * @param {number[]} nums
 * @return {number[]}
 */
function sortedSquaresFast(nums) {
    const vec = new Array(nums.length);

    let start = 0;
    let end = nums.length - 1;
    
    for (let idx = nums.length - 1; idx > -1; idx--) {
        if (Math.abs(nums[end]) > Math.abs(nums[start])) {
            vec[idx] = nums[end] * nums[end];
            end--;
        } else {
            vec[idx] = nums[start] * nums[start];
            start++;
        }
    }
    return vec;
};

console.log(sortedSquaresFast([-4,-1,0,3,10])); // [0,1,9,16,100]
