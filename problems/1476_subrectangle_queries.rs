struct SubrectangleQueries {
    data: Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {

    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        SubrectangleQueries { data: rectangle }
    }
    
    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        let (row1, row2, col1, col2): (usize, usize, usize, usize) = (row1 as usize, row2 as usize, col1 as usize, col2 as usize);
        for i in row1..row2 + 1 {
            for j in col1..col2 + 1 {
                self.data[i][j] = new_value;
            }
        }
    }
    
    fn get_value(&self, row: i32, col: i32) -> i32 {
        return self.data[row as usize][col as usize];
    }
}

/**
 * Your SubrectangleQueries object will be instantiated and called as such:
 * let obj = SubrectangleQueries::new(rectangle);
 * obj.update_subrectangle(row1, col1, row2, col2, newValue);
 * let ret_2: i32 = obj.get_value(row, col);
 */
