pub mod lunar;
use lunar::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Date {
    year: i32,
    month: u32,
    day: u32,
}

impl Date {
    fn new(year: i32, month: u32, day: u32) -> Option<Self> {
        if !(1..=12).contains(&month) || !(1..=Self::days_in_month(year, month)).contains(&day) {
            return None;
        }
        Some(Date { year, month, day })
    }

    fn from_str(date_str: &str) -> Option<Self> {
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 3 {
            return None;
        }

        let year = parts[0].parse::<i32>().ok()?;
        let month = parts[1].parse::<u32>().ok()?;
        let day = parts[2].parse::<u32>().ok()?;

        Date::new(year, month, day)
    }

    fn is_leap_year(year: i32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if Self::is_leap_year(year) { 29 } else { 28 },
            _ => 0
        }
    }

    fn days_from_year_start(&self) -> u32 {
        let mut total_days = self.day;
        for month in 1..self.month {
            total_days += Self::days_in_month(self.year, month);
        }
        total_days
    }

    fn days_in_year(&self) -> u32 {
        if Self::is_leap_year(self.year) { 366 } else { 365 }
    }

    fn day_of_week(&self) -> u8 {
        let century = self.year / 100;
        let c = (3 - century % 4) * 2 % 7;

        let year_in_century = self.year % 100;
        let mut y = (year_in_century + year_in_century / 4) % 7;

        if Self::is_leap_year(self.year) && (self.month <= 2) {
            y -= 1;
        }

        let m = match self.month {
            1 => 0,
            2 => 3,
            3 => 3,
            4 => 6,
            5 => 1,
            6 => 4,
            7 => 6,
            8 => 2,
            9 => 5,
            10 => 0,
            11 => 3,
            12 => 5,
            _ => panic!("Invalid month")
        };

        let d = self.day % 7;
        let w = (c + y + m + d as i32) % 7 - 1;

        if w < 0 { (w + 7) as u8 } else { w as u8 }
    }

    // 计算是本年第几天
    fn day_of_year(&self) -> u32 {
        self.days_from_year_start()
    }

    // 判断是否为A股交易日
    fn is_trading_day(&self) -> bool {
        // 周六日不交易
        let weekday = self.day_of_week();
        if weekday == 6 || weekday == 0 {
            return false;
        }

        // 法定节假日不交易 (这里简化处理，实际应该用查表)
        match (self.month, self.day) {
            // 元旦
            (1, 1) => false,
            // 春节假期 (假设7天)
            (m, d) if m == 1 && d >= 21 && d <= 27 => false,
            // 清明节
            (4, 5) => false,
            // 劳动节
            (5, 1) => false,
            // 端午节
            (6, 22) => false,
            // 中秋节
            (9, 29) => false,
            // 国庆节
            (10, 1..=7) => false,
            _ => true,
        }
    }

    // 计算到下一个交易日的天数
    fn days_until_next_trading_day(&self) -> u32 {
        let mut current = *self;
        let mut days = 0;

        loop {
            // 移到下一天
            let next_day = if current.day == Self::days_in_month(current.year, current.month) {
                if current.month == 12 {
                    Date::new(current.year + 1, 1, 1).unwrap()
                } else {
                    Date::new(current.year, current.month + 1, 1).unwrap()
                }
            } else {
                Date::new(current.year, current.month, current.day + 1).unwrap()
            };

            days += 1;
            if next_day.is_trading_day() {
                break;
            }
            current = next_day;
        }

        days
    }
}

struct WeekCalculator;

impl WeekCalculator {
    fn calculate_week_number(date: &Date) -> u32 {
        let first_day = Date::new(date.year, 1, 1).unwrap();
        let first_day_of_next_year = Date::new(date.year + 1, 1, 1).unwrap();

        let first_day_week = first_day.day_of_week();
        let first_day_next_week = first_day_of_next_year.day_of_week();

        let days = date.days_from_year_start();
        let total_days = date.days_in_year();
        let remaining_days = total_days - days + 1;

        if first_day_next_week > remaining_days as u8 {
            return 1;
        }

        let first_weekend = if first_day_week == 6 { 1 } else { 7 - first_day_week };

        if days <= first_weekend as u32 {
            return 1;
        }

        let days_after_first_week = days - first_weekend as u32;

        if first_day_week == 0 {
            return (days - 1) / 7 + 1;
        }

        if days_after_first_week % 7 == 0 {
            if first_day_week > 3 { // Over Tuesday
                return days_after_first_week / 7;
            }
            days_after_first_week / 7 + 1
        } else {
            if first_day_week > 3 { // Over Tuesday
                return days_after_first_week / 7 + 1;
            }
            days_after_first_week / 7 + 2
        }
    }
}

pub fn time_info(time_str: &str) -> String {
    let date = match Date::from_str(time_str) {
        Some(date) => date,
        None => return "Invalid date".to_string(),
    };

    let week_num = WeekCalculator::calculate_week_number(&date);
    let weekday = date.day_of_week();
    let day_of_year = date.day_of_year();
    let remaining_days = date.days_in_year() - date.days_from_year_start();

    let lunar_calendar = LunarCalendar::new();
    let days_to_new_year = match lunar_calendar.days_until_lunar_new_year(&date) {
        Some(days) => days.to_string(),
        None => "不支持过于遥远的日期 =(".to_string()
    };

    let days_to_trading = if date.is_trading_day() {
        0
    } else {
        date.days_until_next_trading_day()
    };

    format!("{},{},{},{},{},{}",
            week_num,
            weekday,
            day_of_year,
            remaining_days,
            days_to_new_year,
            days_to_trading
    )
}

