use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph = HashMap::new();
        for (edge, value) in equations.into_iter().zip(values.into_iter()) {
            graph.entry(edge[0].clone()).or_insert_with(Vec::new).push((edge[1].clone(), value));
            graph.entry(edge[1].clone()).or_insert_with(Vec::new).push((edge[0].clone(), 1.0 / value));
        }
                
        let mut visited = HashSet::new();
        return queries
            .into_iter()
            .map(|query| Self::resolve(&mut graph, &mut visited, &query[0], &query[1]))
            .collect();
    }
    
    fn resolve(
        graph: &mut HashMap<String, Vec<(String, f64)>>, 
        visited: &mut HashSet<String>, 
        origin: &String, 
        destination: &String,
    ) -> f64 {
        if !graph.contains_key(origin) {
            return -1.0;
        }
        if origin == destination {
            return 1.0;
        }
        
        for edge in graph[origin].clone() {
            if visited.contains(&edge.0) {
                continue;
            }
            visited.insert(edge.0.clone());
            let tmp = Self::resolve(graph, visited, &edge.0, destination);
            visited.remove(&edge.0);
            if tmp == -1.0 {
                continue;
            }
            return tmp * edge.1;
        }

        return -1.0;
    }
}
