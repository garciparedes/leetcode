const MODULO: usize = 1_000_000_007;

impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let (n, m, k) = (n as usize, m as usize, k as usize);
        let mut ways = vec![vec![vec![0; k + 1]; m + 1]; n + 1];
        
        for j in 1..m + 1 {
            ways[1][j][1] = 1;
        }
        
        for i in 1..n + 1 {
            for j in 1..m + 1 {
                for l in 1..k + 1 {
                    let mut tmp = (j * ways[i - 1][j][l]) % MODULO;
                    
                    for x in 1..j {
                        tmp = (tmp + ways[i - 1][x][l - 1]) % MODULO;
                    }
                    
                    ways[i][j][l] = (ways[i][j][l] + tmp) % MODULO;
                }
            }
        }
        
        let mut ans = 0;
        for j in 1..m + 1 {
            ans = (ans + ways[n][j][k]) % MODULO;
        }
        return ans as i32;
    }
}
