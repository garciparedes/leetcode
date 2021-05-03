use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let word: Vec<_> = word.chars().collect();
        let mut memo = HashMap::new();
        let mut ans = 0;
        for i in 0..word.len() {
            ans = cmp::max(ans, Self::helper(&word, i, 'a', false, &mut memo));    
        }
        return ans;
        
    }
    
    fn helper(word: &[char], index: usize, current: char, satisfied: bool, memo: &mut HashMap<(usize, char, bool), i32>) -> i32 {
        if let Some(&ans) = memo.get(&(index, current, satisfied)) {
            return ans;
        }
        
        let ans;
        if index < word.len() && word[index] == current {
            let tmp = Self::helper(word, index + 1, current, true, memo);
            if tmp != -1 {
                ans = 1 + tmp;
            } else {
                ans = -1;
            }
        } else if satisfied {
            if let Some(next) = Self::next(current) {
                ans = Self::helper(word, index, next, false, memo)
            } else {
                ans = 0
            }
        } else {
            ans =-1
        }
        
        memo.insert((index, current, satisfied), ans);
        
        return ans;
    }
    
    fn next(curr: char) -> Option<char> {
        match curr {
            'a' => Some('e'),
            'e' => Some('i'),
            'i' => Some('o'),
            'o' => Some('u'),
            _ => None,
        }
    }
}
