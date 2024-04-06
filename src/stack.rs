pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut stack = String::new();
    let mut result = Vec::new();
    let (mut opened, mut closed) = (0, 0);

    fn walk() {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_parenthesis_example_1() {
        let n = 3;
        let output = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        assert_eq!(output, generate_parenthesis(n));
    }

    #[test]
    fn generate_parenthesis_example_2() {
        let n = 1;
        let output = vec!["()"];
        assert_eq!(output, generate_parenthesis(n));
    }
}
