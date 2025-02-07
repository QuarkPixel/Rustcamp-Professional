pub fn new_count_distinct(input_str: &str) -> usize {
    input_str
        .split(',') // 按逗号分割字符串
        .map(|s| s.trim()) // 去除可能的空白字符
        .filter(|s| !s.is_empty()) // 过滤掉空字符串
        .collect::<std::collections::HashSet<_>>() // 收集到 HashSet 中自动去重
        .len() // 获取 HashSet 的长度，即不重复元素的个数
}
