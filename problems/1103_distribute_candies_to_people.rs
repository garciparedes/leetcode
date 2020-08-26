use std::cmp;

impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let turn = num_people * (num_people + 1) / 2;
        
        let mut i = 0;
        let mut initial = 0;
        while ((i + 1) * num_people * ((i + 1) * num_people + 1) / 2) < candies {
            initial += 1 + i * num_people;
            i += 1;
        }
        candies -= i * num_people * (i * num_people + 1) / 2;
                        
        let mut result: Vec<_> = (0..num_people)
            .into_iter()
            .map(|x| initial + x * i)
            .collect();
        
        let mut j = 0;
        while candies > 0 {
            let tmp = cmp::min(candies, (j + 1 +  num_people * i));
            candies -= tmp;
            result[j as usize] += tmp;
            j += 1;
        }
        
        return result;
    }
}

