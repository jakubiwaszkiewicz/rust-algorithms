pub (crate) fn _binary_search(vec: Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = vec.len();
    let mut mid = 0;

    while low <= high {
        mid = (high + low) / 2;

        if vec[mid] < target {
            low = mid + 1;
        } else if vec[mid] > target {
            high = mid - 1;
        } else if vec[mid] == target {
            return mid.try_into().unwrap()
        }
    }
    return -1
}