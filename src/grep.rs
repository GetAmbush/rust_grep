use crate::errors::Error;
use std::collections::VecDeque;

pub struct MatchResult {
    begin: usize,
    end: usize,
}

pub fn run(expression: &String, source: impl Iterator<Item = u8>) -> Vec<MatchResult> {
    let mut match_count: usize = 0;
    let expression_len = expression.as_bytes().len();

    let window: VecDeque<u8> = VecDeque::new();

    // optimize to use extend or similar
    for i in 0..expression_len {
        window.push_back(source.next().unwrap());
    }

    //next window
    window.pop_front();
    window.push_back(source.next().unwrap());

    //for each window
    //if window == expression, match

    match_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        let test = String::from("0123456789678");
        let expression = String::from("678");
        let result = run(&expression, test.bytes().into_iter());
        assert_eq!(result[0].begin, 6);
        assert_eq!(result[0].end, 8);
    }

    #[test]
    fn test2() {
        let test = String::from("121234");
        let expression = String::from("123");
        let result = run(&expression, test.bytes().into_iter());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].begin, 2);
        assert_eq!(result[0].end, 4);
    }

    #[test]
    fn test3() {
        let test = String::from("AAAAAAABBBBBB");
        let expression = String::from("AAAB");
        let result = run(&expression, test.bytes().into_iter());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].begin, 4);
        assert_eq!(result[0].end, 7);
    }

    #[test]
    fn test4() {
        let test = String::from("AAAA");
        let expression = String::from("AA");
        let result = run(&expression, test.bytes().into_iter());

        assert_eq!(result[0].begin, 0);
        assert_eq!(result[0].end, 1);

        assert_eq!(result[1].begin, 1);
        assert_eq!(result[1].end, 2);

        assert_eq!(result[2].begin, 2);
        assert_eq!(result[2].end, 3);
    }
}
