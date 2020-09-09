use std::cmp;
use std::cmp::Ordering;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let version1 = Self::format_version(version1);
        let version2 = Self::format_version(version2);
        
        for i in 0..cmp::max(version1.len(), version2.len()) {
            match version1.get(i).unwrap_or(&0).cmp(&version2.get(i).unwrap_or(&0)) {
                Ordering::Less => return -1,
                Ordering::Equal => (),
                Ordering::Greater => return 1,
            }
        }
        return 0;
    }
    
    fn format_version(version: String) -> Vec<i32> {
        version
            .split('.')
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }
}
