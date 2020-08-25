use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut memory = HashMap::new();
        
        return Self::rec(&costs, &days, 0, &mut memory);
    }
    
    fn rec(costs: &Vec<i32>, days: &Vec<i32>, i: usize, memory: &mut HashMap<usize, i32>) -> i32 {
        if i >= days.len() {
            return 0;
        }
        if let Some(result) = memory.get(&i) {
            return *result;
        }
        
        let result = cmp::min(
            Self::rec_one(&costs, &days, i, memory), 
            cmp::min(Self::rec_seven(&costs, &days, i, memory), Self::rec_thirty(&costs, &days, i, memory))
        );
        memory.insert(i, result);
        return result;
    }
    
    fn rec_one(costs: &Vec<i32>, days: &Vec<i32>, i: usize,  memory: &mut HashMap<usize, i32>) -> i32 {
        let end = days[i] + 1;
        let mut one = costs[0];
        let mut j = i + 1;
        while j < days.len()  && days[j] < end {
            j += 1;
        }
        one += Self::rec(costs, days, j, memory);
        return one;
    }
    
    fn rec_seven(costs: &Vec<i32>, days: &Vec<i32>, i: usize, memory: &mut HashMap<usize, i32>) -> i32 {
        let end = days[i] + 7;
        let mut seven = costs[1];
        let mut j = i + 1;
        while j < days.len() && days[j] < end {
            j += 1;
        }
        seven += Self::rec(costs, days, j, memory);
        return seven;
    }
    
    fn rec_thirty(costs: &Vec<i32>, days: &Vec<i32>, i: usize, memory: &mut HashMap<usize, i32>) -> i32 {
        let end = days[i] + 30;
        let mut thirty = costs[2];
        let mut j = i + 1;
        while j < days.len() && days[j] < end {
            j += 1;
        }
        thirty += Self::rec(costs, days, j, memory);
        return thirty;
    }
        
}
