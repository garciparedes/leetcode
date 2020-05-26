impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut operations = Vec::new();
        let mut permutation : Vec<i32> = (1..m + 1).collect();
        for q_i in queries {
            let mut j = 0;
            while q_i != permutation[j] {
                j += 1;
            }
            let item =  permutation.remove(j);
            permutation.insert(0, item);
            operations.push(j as i32);
        }
        return operations;
    }
}
