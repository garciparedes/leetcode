const DIRECTIONS: &'static[&'static [i32]] = &[&[0, 1], &[-1, 0], &[0, -1], &[1, 0]];

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut position = vec![0, 0];
        let mut index: isize = 0;
        for instruction in instructions.chars() {
            match instruction {
                'G' => {
                    position[0] += DIRECTIONS[index as usize][0];
                    position[1] += DIRECTIONS[index as usize][1];
                },
                'L' => {
                    index = (index + 1) % 4;
                },
                'R' => {
                    index = ((index - 1) % 4 + 4) % 4;
                },
                _ => panic!(),
            }
        }
        return ((position[0] == 0) && (position[1] == 0)) || (index != 0);
    }
}
