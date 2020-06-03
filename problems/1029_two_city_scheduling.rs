impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let n = costs.len() / 2;        
        
        let total_a: i32 = costs.iter().map(|x| x[0]).sum();
        let mut refund: Vec<i32> = costs.iter().map(|x| x[1] - x[0]).collect();
        refund.sort();
        
        return total_a + refund[0..n].iter().sum::<i32>();
    }
}
