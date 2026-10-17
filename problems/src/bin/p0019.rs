fn is_leap(year: u32) -> bool {
    year % 4 == 0 && (year % 400 == 0 || year % 100 != 0)
}

enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Month {
    fn num_days(&self, year: u32) -> u32 {
        match self {
            Self::Jan => 31,
            Self::Feb if is_leap(year) => 29,
            Self::Feb => 28,
            Self::Mar => 31,
            Self::Apr => 30,
            Self::May => 31,
            Self::Jun => 30,
            Self::Jul => 31,
            Self::Aug => 31,
            Self::Sep => 30,
            Self::Oct => 31,
            Self::Nov => 30,
            Self::Dec => 31,
        }
    }

    fn roll(&mut self, year: u32) -> (u32, u32) {
        let days = self.num_days(year);
        let (next, year) = match self {
            Self::Jan => (Self::Feb, year),
            Self::Feb => (Self::Mar, year),
            Self::Mar => (Self::Apr, year),
            Self::Apr => (Self::May, year),
            Self::May => (Self::Jun, year),
            Self::Jun => (Self::Jul, year),
            Self::Jul => (Self::Aug, year),
            Self::Aug => (Self::Sep, year),
            Self::Sep => (Self::Oct, year),
            Self::Oct => (Self::Nov, year),
            Self::Nov => (Self::Dec, year),
            Self::Dec => (Self::Jan, year + 1),
        };
        *self = next;
        (days, year)
    }
}

fn main() {
    let mut day_num = 0u32;
    let mut month = Month::Jan;
    let mut year = 1900u32;
    let mut result = 0u32;
    while year < 2001 {
        if day_num % 7 == 6 && year > 1900 {
            result += 1;
        }
        let (days, new_year) = month.roll(year);
        year = new_year;
        day_num += days;
    }
    println!("{result}");
}
