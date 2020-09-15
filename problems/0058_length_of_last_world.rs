impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut iterable = s.chars().rev().peekable();
        while let Some(item) = iterable.peek() {
            if *item != ' ' {
                break;
            }
            iterable.next();
        }
        let mut ans = 0;
        while let Some(item) = iterable.next() {
            if item == ' ' {
                break;
            }
            ans += 1;
        }
        return ans;
    }
}
