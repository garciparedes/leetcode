impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        if n == 0 {
            return true;
        }
        let m = flowerbed.len();
        for j in 0..m {
            if !(j == 0 || flowerbed[j - 1] == 0) {
                continue;
            }
            if !(flowerbed[j] == 0) {
                continue;
            }
            if !(j == m - 1 || flowerbed[j + 1] == 0) {
                continue;
            }
            flowerbed[j] = 2;
            
            n -= 1;
            if n == 0 {
                return true;
            }
        }
        return false;
    }
}
