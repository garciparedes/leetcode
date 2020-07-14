impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ops = Vec::new();
        let mut current = 0;
        for item in target {
            current += 1;
            ops.push(String::from("Push"));
            while current < item {
                current += 1;
                ops.push(String::from("Pop"));
                ops.push(String::from("Push"));
            }
        }
        return ops;
    }
}
