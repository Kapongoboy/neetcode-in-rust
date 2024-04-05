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
}
