impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..n + 1)
            .into_iter()
            .map(|x| {
                if x % 15 == 0 {
                    String::from("FizzBuzz")
                } else if x % 3 == 0 {
                    String::from("Fizz")
                } else if x % 5 == 0 {
                    String::from("Buzz")
                } else {
                    format!("{}", x)
                }
            })
            .collect()
    }   
}
