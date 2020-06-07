impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut coins: Vec<usize> = coins.into_iter().map(|x| x as usize).collect();
        coins.sort();
       
        let (n, m) = (coins.len() + 1, (amount + 1) as usize);
        let mut table = vec![vec![0; m]; n];

        for i in 0..n {
            table[i][0] = 1;
        }

        for i in 1..n {
            for j in 0..m {
                table[i][j] = table[i - 1][j];
                if j >= coins[i - 1] {
                    table[i][j] += table[i][j - coins[i - 1]];
                }
            }
        }
        
        return table[n - 1][m - 1];
    }
}

