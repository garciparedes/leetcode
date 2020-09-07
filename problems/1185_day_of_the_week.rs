const MONTH_DAYS: &[i32] = &[
    31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
];

const DAY_LABELS: &[&str] = &[
    "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday", 
];

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let days = Self::days_since_start(day, month, year);
        let day_index = (((days - 4) % 7 + 7) % 7) as usize;
        return String::from(DAY_LABELS[day_index]);
    }
    
    fn days_since_start(day: i32, month: i32, year: i32) -> i32 {
        let mut count = 0;
        for y in 1971..year {
            count += 365 + Self::is_leap_year(y) as i32;
        }
        for m in 0..month - 1 {
            count += MONTH_DAYS[m as usize] + (m == 1 && Self::is_leap_year(year)) as i32;
        }
        count += day;
        return count;
    }
    
    fn is_leap_year(year: i32) -> bool {
        ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0)
    }
    
}
