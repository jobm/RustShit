pub fn find_in_arr(arr: Vec<u32>, n: u32) -> i32 {
    let mut low = 0;
    let mut high = arr.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        if arr[mid] == n {
            return mid as i32;
        }
        // kep
        if arr[mid] < n {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    -1
}
