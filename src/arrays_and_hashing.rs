use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut m = HashMap::new();
    for num in nums.iter() {
        m.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut result = m.iter().filter(|v| *v.1 > 1);
    match result.next() {
        Some(_) => true,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_duplicate_example_1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(true, contains_duplicate(nums))
    }

    #[test]
    fn contains_duplicate_example_2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(false, contains_duplicate(nums))
    }

    #[test]
    fn contains_duplicate_example_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(true, contains_duplicate(nums))
    }
}
