impl Solution {
    pub fn sorted_squares(mut a: Vec<i32>) -> Vec<i32> {        
        a.iter_mut().for_each(|x| *x = x.pow(2));
        a.sort();
        return a;
    }
}
