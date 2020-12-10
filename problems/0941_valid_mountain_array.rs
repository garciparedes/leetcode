impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let n = arr.len();
        if n < 3 {
            return false;
        }
        
        let mut i = 0;
        while i < arr.len() - 2 {
            if arr[i] >= arr[i + 1] {
                break;
            }
            i += 1;
        }
        
        for j in i..n - 1 {
            if arr[j] <= arr[j + 1] {
                return false;
            }
        }
        return  arr[0] < arr[i] && arr[i] > arr[n - 1];
    }
}
