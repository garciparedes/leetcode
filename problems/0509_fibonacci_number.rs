impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let (mut tmp_1, mut tmp_2)  = (1, 0);
        let mut current = 0;
        for i in 1..n {
            current = tmp_1 + tmp_2;
            tmp_2 = tmp_1;
            tmp_1 = current;
        }
        return current;
    }
}
