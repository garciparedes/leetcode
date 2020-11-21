use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len() as i32;
        
        let (mut i, mut j) = (0, n - 1);
        while i < j {
            let mid = (i + j) / 2;
            if nums[mid as usize] > nums[j as usize] {
                i = mid + 1;
            } else {
                j = mid;
            }
        }
        let rot = i;
        i = 0;
        j = n - 1;
        while i <= j {
            let mid = (i + j) / 2;
            let real_mid = (mid + rot) % n;
            match nums[real_mid as usize].cmp(&target) {
                Ordering::Less => { i = mid + 1 },
                Ordering::Equal => return real_mid as i32,
                Ordering::Greater => { j = mid - 1 },
            } 
        }
        return -1;
    }
}
