impl Solution {
    pub fn count_students(mut students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        for sand in sandwiches {
            if students.is_empty() || students.iter().all(|stu| *stu != sand) {
                break;
            }
            while let stu = students.remove(0) {
                if stu == sand {
                    break
                }
                students.push(stu);
            }            
        }
        return students.len() as i32;
    }
}
