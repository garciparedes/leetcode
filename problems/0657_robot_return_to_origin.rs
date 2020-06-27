impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let (mut horizontal, mut vertical) = (0, 0);
        
        for op in moves.chars() {
            match op {
                'U' => vertical += 1,
                'D' => vertical -= 1,
                'R' => horizontal += 1,
                'L' => horizontal -= 1,
                _ => (),
            }
        }
        
        return horizontal == 0 && vertical == 0;
    }
}
