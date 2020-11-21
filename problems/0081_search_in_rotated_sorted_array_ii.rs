use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let n = nums.len() as i32;
        let (mut left, mut right) = (0, n - 1);
        
        while left <= right {
            let mut mid = (left + right) / 2;
            if nums[mid as usize] == target {
                return true;
            }
            
            match nums[left as usize].cmp(&nums[mid as usize]) {
                Ordering::Less => {
                    if target < nums[left as usize] || target > nums[mid as usize] {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                },
                Ordering::Equal => { 
                    left += 1
                },
                Ordering::Greater => {
                    if target < nums[mid as usize] || target > nums[right as usize] {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                },
            }
        }
        return false;
    }
}
