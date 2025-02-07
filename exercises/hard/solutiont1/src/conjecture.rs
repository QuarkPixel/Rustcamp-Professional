pub fn goldbach_conjecture() -> String {
    const MAX: usize = 10000; // 答案在10000以内

    // 预处理素数
    let mut is_prime = [true; MAX];
    is_prime[0] = false;
    is_prime[1] = false;
    let sqrt = (MAX as f64).sqrt() as usize;
    for i in 2..=sqrt {
        if is_prime[i] {
            let mut j = i * i;
            while j < MAX {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    // 预处理平方的两倍
    let sqrt_limit = ((MAX as f64) / 2.0).sqrt() as usize;
    let double_squares: Vec<u64> = (0..=sqrt_limit).map(|k| (2 * k * k) as u64).collect();

    // 主要搜索逻辑
    let mut found = Vec::with_capacity(2);
    let mut n = 3;

    'outer: while found.len() < 2 {
        // 只考虑奇合数
        if n % 2 == 1 && !is_prime[n as usize] {
            // 检查是否可以表示为素数加上平方的两倍
            for &double_square in &double_squares {
                if double_square >= n {
                    break;
                }
                let remaining = n - double_square;
                if remaining < MAX as u64 && is_prime[remaining as usize] {
                    n += 2;
                    continue 'outer;
                }
            }
            found.push(n);
        }
        n += 2;
    }

    format!("{},{}", found[0], found[1])
}
