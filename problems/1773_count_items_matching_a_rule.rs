impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let j = match &rule_key[..] {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => panic!(),
        };
        
        let mut ans = 0;
        for item in items {
            if item[j] == rule_value {
                ans += 1;
            }
        }
        return ans;
    }
}
