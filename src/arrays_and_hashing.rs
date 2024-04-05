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

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let pair = target - num;
        match m.get(&pair) {
            Some(idx) => return vec![*idx as i32, i as i32],
            None => m.insert(*num, i),
        };
    }

    vec![0]
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

    #[test]
    fn two_sums_example_1() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let output = vec![0, 1];
        assert_eq!(output, two_sum(input, target));
    }

    #[test]
    fn two_sums_example_2() {
        let input = vec![3, 2, 4];
        let target = 6;
        let output = vec![1, 2];
        assert_eq!(output, two_sum(input, target));
    }

    #[test]
    fn two_sums_example_3() {
        let input = vec![3, 3];
        let target = 6;
        let output = vec![0, 1];
        assert_eq!(output, two_sum(input, target));
    }
}
