static MONTH_MAPPER: &[&str] = &[
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
];

impl Solution {
    pub fn reformat_date(date: String) -> String {

        let divided: Vec<_> = date.split(" ").collect();
        let (day, month, year) = (divided[0], divided[1], divided[2]);
        
        let day = day.chars().filter(|x| x.is_digit(10)).collect::<String>().parse::<u8>().unwrap();
        let month = MONTH_MAPPER.iter().position(|&r| r == month).unwrap() + 1;
        let year = year.parse::<u16>().unwrap();
        
        return format!("{:0>4}-{:0>2}-{:0>2}", year, month, day);
    }
}
