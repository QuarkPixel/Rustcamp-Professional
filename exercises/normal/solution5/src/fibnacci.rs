pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    std::iter::successors(Some((0, 1)), |&(a, b)| Some((b, a + b)))
        .take_while(|&(a, _)| a < threshold)  // 限制数值小于 threshold
        .filter(|&(a, _)| a % 2 == 1)         // 只保留奇数
        .map(|(a, _)| a)                      // 提取斐波那契数值
        .sum()                                 // 求和
}
