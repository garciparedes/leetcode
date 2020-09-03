use std::cmp;

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut scores = vec![0; 2];

        for operation in ops {
            match operation.as_str() {
                "+" => {
                    let score = scores[(scores.len() - 2)..scores.len()]
                            .iter()
                            .sum::<i32>();
                    scores.push(score);
                },
                "D" => {
                    let score = scores[scores.len() - 1] * 2;
                    scores.push(score);    
                },
                "C" => {
                    if scores.len() > 2 {
                        scores.pop();   
                    }
                },
                _ => {
                    let score = operation.parse::<i32>().unwrap();
                    scores.push(score);
                } 
            }
        }
        return scores.into_iter().sum::<i32>();
    }
}
