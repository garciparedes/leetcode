impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n == 1 {
            return String::from("a");
        }
        
        let odd = n % 2 == 1;
        
        let mut result: String = (0..n - (1 + odd as i32)).map(|_| 'a').collect();
        result.push('b');

        if odd {
            result.push('c');
        }
        return result;

    }
}
