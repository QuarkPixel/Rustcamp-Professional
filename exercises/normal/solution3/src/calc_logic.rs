const TOTAL_DAYS: u32 = 365;
pub fn new_birthday_probability(n: u32) -> f64 {
    if n >= TOTAL_DAYS {
        1f64
    } else {
        let n = n - 1;
        1f64 - (TOTAL_DAYS - n..TOTAL_DAYS).map(|x| x as f64)
            .product::<f64>() / (TOTAL_DAYS as f64).powi(n as _)
    }
}
