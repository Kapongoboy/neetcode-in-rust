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
    let mut ordered_s = s.chars().collect::<Vec<char>>();
    ordered_s.sort();
    let mut ordered_t = t.chars().collect::<Vec<char>>();
    ordered_t.sort();
    ordered_s == ordered_t
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
