use std::collections::HashMap;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph: HashMap<i32,Vec<(i32, i32)>> = HashMap::new();
        
        for flight in flights {
            graph.entry(flight[0]).or_insert_with(Vec::new).push((flight[1], flight[2]));
        }
        let mut memory = HashMap::new();
        let cost = Solution::dfs(&graph, vec![src], dst, k as usize, &mut memory);
        
        if cost == i32::max_value() {
            return -1;
        }

        return cost;
    }


    fn dfs(
        graph: &HashMap<i32, Vec<(i32, i32)>>, 
        path: Vec<i32>,
        dst: i32, 
        k: usize, 
        memory: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if path.len() > k + 2 {
            return i32::max_value();
        }
        let current = path[path.len() - 1];
        
        if current == dst {
            return 0;
        }
        if !graph.contains_key(&current) {
            return i32::max_value();
        }

        let iterable = graph[&current].iter().filter(|x| path.iter().any(|&y| y != x.0));

        let mut best_cost = i32::max_value();
        for alternative in iterable {
            let mut current_path = path.clone();
            current_path.push(alternative.0);
            if alternative.1 >= best_cost {
                continue;
            }
            let memory_identifier = (current_path.len(), alternative.0);
            let deep_cost = match memory.get(&memory_identifier) {
                Some(&x) => x,
                None =>  Solution::dfs(graph, current_path, dst, k, memory),
            };
            memory.insert(memory_identifier, deep_cost);
            if deep_cost >= best_cost - alternative.1 {
                continue;
            }
            best_cost = deep_cost + alternative.1;
        }
        return best_cost;
    }
}
