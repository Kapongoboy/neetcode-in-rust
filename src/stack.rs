pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let end = temperatures.len() - 1;

    fn walk(res: &mut Vec<i32>, temps: &Vec<i32>, curr: usize, end: usize) {
        if curr >= end {
            res.push(0);
            return;
        }

        let mut count = 0;

        for i in curr..temps.len() {
            if i == end && temps[i] <= temps[curr] {
                res.push(0);
                break;
            }

            if temps[i] > temps[curr] {
                res.push(count);
                break;
            } else {
                count += 1;
            }
        }
        walk(res, temps, curr + 1, end);
    }

    walk(&mut result, &temperatures, 0, end);
    result
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    fn walk(opened: i32, closed: i32, s: String, result: &mut Vec<String>) {
        if opened == 0 && closed == 0 {
            result.push(s);
            return;
        }

        if opened == closed {
            walk(opened - 1, closed, s.clone() + "(", result);
        } else {
            if opened > 0 {
                walk(opened - 1, closed, s.clone() + "(", result);
            }

            if closed > 0 {
                walk(opened, closed - 1, s.clone() + ")", result);
            }
        }
    }

    walk(n, n, String::from(""), &mut result);
    result
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

    #[test]
    fn daily_temperatures_example_1() {
        let input: Vec<i32> = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let output: Vec<i32> = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(output, daily_temperatures(input));
    }

    #[test]
    fn daily_temperatures_example_2() {
        let input: Vec<i32> = vec![30, 40, 50, 60];
        let output: Vec<i32> = vec![1, 1, 1, 0];
        assert_eq!(output, daily_temperatures(input));
    }

    #[test]
    fn daily_temperatures_example_3() {
        let input: Vec<i32> = vec![30, 60, 90];
        let output: Vec<i32> = vec![1, 1, 0];
        assert_eq!(output, daily_temperatures(input));
    }
}
