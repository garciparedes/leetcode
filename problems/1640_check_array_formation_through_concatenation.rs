impl Solution {
    pub fn can_form_array(arr: Vec<i32>, mut pieces: Vec<Vec<i32>>) -> bool {
        let mut i = 0;
        while i < arr.len() {
            
            let mut matched = false;
            for j in 0..pieces.len() {
                if arr[i] == pieces[j][0] {
                    i += 1;
                    for k in 1..pieces[j].len() {
                        if arr[i] == pieces[j][k] {
                            i += 1;
                        } else {
                            return false;
                        }
                    }
                    pieces.remove(j);
                    matched = true;
                    break;
                }   
            }
            if !matched {
                return false;
            }
        }
        
        return true;
    }
}
