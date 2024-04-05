use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut m = HashMap::new();
    for num in nums.iter() {
        match m.get(num) {
            Some(_) => return true,
            None => m.insert(num, 0),
        };
    }

    false
}

pub fn is_anagram(s: String, t: String) -> bool {
    let mut m_s = HashMap::new();
    let mut m_t = HashMap::new();

    for item in s.chars() {
        m_s.entry(item).and_modify(|i| *i += 1).or_insert(1);
    }

    for item in t.chars() {
        m_t.entry(item).and_modify(|i| *i += 1).or_insert(1);
    }

    m_s == m_t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_duplicate_example_1() {
        let nums = vec![1, 2, 3, 1];
        assert!(contains_duplicate(nums))
    }

    #[test]
    fn contains_duplicate_example_2() {
        let nums = vec![1, 2, 3, 4];
        assert!(!contains_duplicate(nums))
    }

    #[test]
    fn contains_duplicate_example_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert!(contains_duplicate(nums))
    }

    #[test]
    fn is_anagram_example_1() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        assert!(is_anagram(s, t));
    }

    #[test]
    fn is_anagram_example_2() {
        let s = String::from("rat");
        let t = String::from("car");
        assert!(!is_anagram(s, t));
    }
}
