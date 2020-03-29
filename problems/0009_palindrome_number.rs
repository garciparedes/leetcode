struct Solution {}

fn main() {
    let input = 1321;
    let output = Solution::is_palindrome(input);
    println!("The solution of \"{}\" is {}", input, output)
}


impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let characters = x.to_string();
        let iterable = characters.chars()
            .zip(characters.chars().rev())
            .take(characters.chars().count() / 2 + 1);

        for (x, y) in iterable {
            if x != y {
                return false;
            }
        }
        return true;
    }
}
