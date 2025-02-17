/*
    Anagram Check
    Given two strings, check if they are anagrams of each other.
    Anagrams are words or phrases formed by rearranging the letters of another,
    using all the original letters exactly once.
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::fmt::{self, Display, Formatter};

fn main() {}

pub fn are_anagrams(s1: String, s2: String) -> bool {
    // 定义一个辅助函数来归一化字符串
    fn normalize(s: &str) -> String {
        s.chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect()
    }

    // 对两个输入字符串进行归一化处理
    let norm_s1 = normalize(&s1);
    let norm_s2 = normalize(&s2);

    // 将归一化后的字符串转换为字符向量
    let mut chars_s1: Vec<char> = norm_s1.chars().collect();
    let mut chars_s2: Vec<char> = norm_s2.chars().collect();

    // 对字符向量进行排序
    chars_s1.sort();
    chars_s2.sort();

    // 比较排序后的字符向量是否相等
    chars_s1 == chars_s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}
