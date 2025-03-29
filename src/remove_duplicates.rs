pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // given [1,1,1,2,2,3,4] - [1,1,2,2,3,4]
    let mut count = 0; // Count occurrences of the current number
    let mut nxt_idx = 0; // Pointer to where we write the next valid element

    for curr_idx in 0..nums.len() {
        if curr_idx == 0 || nums[curr_idx] != nums[curr_idx - 1] {
            // Reset count for a new number
            count = 1;
        } else {
            // Increment count for the same number
            count += 1;
        }

        if count <= 2 {
            // Write the number only if it appears at most twice
            nums[nxt_idx] = nums[nxt_idx];
            nxt_idx += 1;
        }
    }
    nxt_idx as i32
}
