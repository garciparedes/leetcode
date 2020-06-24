impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut array = a.clone();
        let mut threshold = array.len();
        let mut i = 0;
        
        while i < threshold {
            if array[i] % 2 == 0 {
                i += 1;
            } else {
                let item = array.remove(i);
                array.push(item);
                threshold -= 1;
            }
        }
        return array;
    }
}
