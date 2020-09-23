impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut cum = 0;
        let (mut start, mut tank) = (0, 0);
        for i in 0..n {
            let tmp = gas[i] - cost[i];
            cum += tmp;
            tank += tmp; 
            if tank < 0 {
                tank = 0;
                start = i + 1;
            }
        }
        if cum < 0 {
            return -1;
        }
            
        return start as i32;
    }
}
