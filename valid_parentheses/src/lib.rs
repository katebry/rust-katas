use regex::Regex;

pub fn valid_parentheses(parens: &str) -> bool {
    match Regex::new(parens){
        Ok(_)  => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(expected: bool, input: &str) {
        assert_eq!(valid_parentheses(input), expected, "\nYour result (left) did not match the expected output (right) for the input: {input:?}");
    }

    #[test]
    fn valid_cases() {
        do_test(true, "()");
        do_test(true, "((()))");
        do_test(true, "()()()");
        do_test(true, "(()())()");
        do_test(true, "()(())((()))(())()");
    }

    #[test]
    fn invalid_cases()  {
        do_test(false, ")(");
        do_test(false, "()()(");
        do_test(false, "((())");
        do_test(false, "())(()");
        do_test(false, ")()");
        do_test(false, ")");
    }

    #[test]
    fn empty_string() {
        do_test(true, "");
    }
}
