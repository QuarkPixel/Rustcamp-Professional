struct BaseConversion {
    value: u32,
}

impl BaseConversion {
    fn from(num: &str, base: u32) -> Self {
        let value = num.chars().rev().enumerate()
            .map(|(i, x)| {
                x.to_digit(base).expect("Invalid number") *
                    base.pow(i as u32)
            }).sum();
        Self { value }
    }

    fn to(&self, target_base: u32) -> String {
        let mut value = self.value;
        let mut result = String::new();

        while value > 0 {
            let digit = value % target_base;
            result.push(digit_to_char(digit).unwrap());
            value /= target_base;
        }

        if result.is_empty() {
            "0".to_string()
        } else {
            result.chars().rev().collect()
        }
    }
}

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let (num, base) = parse_input(num_str);
    let value = BaseConversion::from(num, base);
    value.to(to_base)
}

fn parse_input(input: &str) -> (&str, u32) {
    let parts = input.split_once('(').unwrap();

    // Get the numeric part
    let num = parts.0;

    // Get the base part
    let base = parts.1.trim_end_matches(')')
        .parse::<u32>().expect("Invalid base");

    (num, base)
}

fn digit_to_char(digit: u32) -> Option<char> {
    match digit {
        0..=9 => Some(('0' as u8 + digit as u8) as char),
        10..=35 => Some(('a' as u8 + (digit - 10) as u8) as char),
        _ => None,
    }
}