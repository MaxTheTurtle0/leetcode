
// easy approach
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut vec = nums
        .iter()
        .map(|num| num * num)
        .collect::<Vec<i32>>();
    vec.sort_unstable();
    return vec;
}

// O(n) approach
pub fn sorted_squares_fast(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut vec = vec![0; n];
    
    let (mut start, mut end) = (0, n - 1);

    for idx in (0..n).rev()  { 
        if nums[end].abs() > nums[start].abs() {
            vec[idx] = nums[end] * nums[end];
            end -= 1;
        } else {
            vec[idx] = nums[start] * nums[start];
            start += 1;
        }      
    }
    return vec;
}
