impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return nums;
        }
        
        let mut nums = nums.clone();
        nums.sort_by_key(|x| -x);
        
        let mut groups: Vec<Vec<i32>> = Vec::new();
        
        for elem in nums {
            let mut index: i32 = -1;
            for i in 0..groups.len() {
                if groups[i][groups[i].len() - 1] % elem == 0 {
                    index = i as i32;
                    break;
                }
            }
            if index != -1 {
                let mut group = groups[index as usize].clone();
                group.push(elem);
                groups.push(group);
            } else {
                groups.push(vec![elem]);
            }
            groups.sort_by_key(|x| -(x.len() as i32));
        }
        return groups.into_iter().max_by_key(|group| group.len()).unwrap();
    }
}
