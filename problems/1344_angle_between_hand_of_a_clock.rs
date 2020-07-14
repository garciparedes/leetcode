use std::cmp;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let (hour, minutes) = (hour as f64, minutes as f64);
        
        let a =  30.0 * (hour + minutes / 60.0) % 360.0;
        let b = 6.0 * minutes % 360.0;
        
        let mut diff = f64::abs(a - b);
        if diff > 360.0 - diff {
            diff = 360.0 - diff;
        }
        return diff;
    }
}
