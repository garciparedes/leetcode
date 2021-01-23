impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() + 1;
        let mut ans = vec![0; n];
        
        let mut initial = 0;
        for i in 1..=(n as i32) {
            let mut found = true;
            let mut tmp = i;
            for j in 0..encoded.len() {
                tmp ^= encoded[j];
                if tmp == 0 || tmp > n as i32 {
                    found = false;
                    break;
                }
            }    
            if found {
                initial = i;
                break;
            }
        }
        
        let mut ans = vec![initial];
        for i in 0..encoded.len() {
            let tmp = ans[ans.len() - 1] ^ encoded[i];
            ans.push(tmp);
        }
        
        return ans;
        
        
    }
}
