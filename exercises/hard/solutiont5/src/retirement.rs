use chrono::{Datelike, NaiveDate};
use std::str::FromStr;

// 退休政策配置
#[derive(Debug)]
struct RetirementPolicy {
    original_age: i32,   // 原退休年龄
    max_delay: i32,      // 最大延迟月数
    delay_interval: i32, // 延迟间隔(月)
}

// 退休人员类别
#[derive(Debug)]
enum RetirementCategory {
    Male,             // 男职工
    FemaleManagerial, // 原法定退休年龄55周岁女职工
    FemaleOrdinary,   // 原法定退休年龄50周岁女职工
}

impl RetirementCategory {
    fn get_policy(&self) -> RetirementPolicy {
        match self {
            RetirementCategory::Male => RetirementPolicy {
                original_age: 60,
                max_delay: 36,
                delay_interval: 4,
            },
            RetirementCategory::FemaleManagerial => RetirementPolicy {
                original_age: 55,
                max_delay: 36,
                delay_interval: 4,
            },
            RetirementCategory::FemaleOrdinary => RetirementPolicy {
                original_age: 50,
                max_delay: 60,
                delay_interval: 2,
            },
        }
    }
}

struct RetirementResult {
    retirement_date: NaiveDate,
    retirement_age: f64,
    delay_months: i32,
}

impl RetirementResult {
    fn format(&self) -> String {
        let age_str = if self.retirement_age.fract() == 0.0 {
            format!("{}", self.retirement_age.trunc())
        } else {
            format!("{:.2}", self.retirement_age)
        };

        format!(
            "{},{},{}",
            self.retirement_date.format("%Y-%m"),
            age_str,
            self.delay_months
        )
    }
}

struct RetirementCalculator {
    policy_start_date: NaiveDate,
}

impl RetirementCalculator {
    fn new() -> Self {
        Self {
            policy_start_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        }
    }

    fn calculate(&self, birth_date: NaiveDate, category: RetirementCategory) -> RetirementResult {
        let policy = category.get_policy();

        // 原始退休日期
        let original_retirement_date = birth_date
            .clone()
            .with_year(birth_date.year() + policy.original_age)
            .unwrap();

        use std::cmp::Ordering;
        match original_retirement_date.cmp(&self.policy_start_date) {
            Ordering::Less => RetirementResult {
                retirement_date: original_retirement_date,
                retirement_age: policy.original_age as f64,
                delay_months: 0,
            },
            Ordering::Equal => RetirementResult {
                retirement_date: original_retirement_date
                    .clone()
                    .with_month(original_retirement_date.day() + 1)
                    .unwrap(),
                retirement_age: policy.original_age as f64 + (1.0 / 12.0),
                delay_months: 1,
            },
            Ordering::Greater => {
                // 计算需要延迟的月数
                let months_after_policy =
                    Self::months_between(self.policy_start_date, original_retirement_date);
                let mut delay_months =
                    (months_after_policy + policy.delay_interval - 1) / policy.delay_interval;

                // 限制最大延迟月数
                delay_months = delay_months.min(policy.max_delay);

                // 计算最终退休日期
                let final_retirement_date =
                    Self::add_months(original_retirement_date, delay_months);

                // 计算实际退休年龄
                let total_months = Self::months_between(birth_date, final_retirement_date);
                let retirement_age = total_months as f64 / 12.0;

                RetirementResult {
                    retirement_date: final_retirement_date,
                    retirement_age,
                    delay_months,
                }
            }
        }
    }

    // 计算两个日期之间的月数
    fn months_between(start: NaiveDate, end: NaiveDate) -> i32 {
        (end.year() - start.year()) * 12 + (end.month() as i32 - start.month() as i32)
    }

    // 日期加上指定月数
    fn add_months(date: NaiveDate, months: i32) -> NaiveDate {
        let total_months = date.year() * 12 + date.month() as i32 + months;
        let new_year = total_months / 12;
        let new_month = total_months % 12;
        NaiveDate::from_ymd_opt(
            if new_month == 0 {
                new_year - 1
            } else {
                new_year
            },
            if new_month == 0 { 12 } else { new_month as u32 },
            1,
        )
        .unwrap()
    }
}

pub fn retire_time(time: &str, tp: &str) -> String {
    let category = match tp {
        "男职工" => RetirementCategory::Male,
        "原法定退休年龄50周岁女职工" => RetirementCategory::FemaleOrdinary,
        "原法定退休年龄55周岁女职工" => RetirementCategory::FemaleManagerial,
        _ => panic!("非法人员类型！\nIllegal personnel type!"),
    };

    let date_str = format!("{}-01", time);
    let birth_date = NaiveDate::from_str(&date_str).expect("Invalid date format");

    let calculator = RetirementCalculator::new();
    let result = calculator.calculate(birth_date, category);

    result.format()
}
