pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    fn walk(opened: i32, closed: i32, s: String, result: &mut Vec<String>) {
        if opened == 0 && closed == 0 {
            result.push(s);
            return;
        }

        if opened ==  closed {
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
}
