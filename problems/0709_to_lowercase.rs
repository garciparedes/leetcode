impl Solution {
    pub fn to_lower_case(str: String) -> String {
        return str.chars().map(|x| x.to_lowercase().collect::<String>()).collect::<String>();
    }
}
