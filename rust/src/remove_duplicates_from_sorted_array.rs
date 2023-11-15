pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut previous_num = nums[0];
    let mut is_first = true;

    nums.retain(|&num| {
        if is_first {
            is_first = false;
            return true;
        }

        let should_retain = num != previous_num;
        previous_num = num;
        return should_retain;
    });

    return nums.len() as i32;
}
