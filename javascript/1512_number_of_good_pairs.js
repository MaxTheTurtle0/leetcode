/**
 * @param {number[]} nums
 * @return {number}
 */
function numIdenticalPairs(nums) {
    let numPairs = 0;
    for (let i = 0; i < nums.length; i++) {
        for (let j = i + 1; i < j && j < nums.length; j++) {
            console.log(nums[i], nums[j]); 
            if (nums[i] === nums[j]) {
                numPairs++;
            }
        }
    }   
    return numPairs;
};
