use crate::errors::Error;
use std::collections::VecDeque;

pub struct MatchResult {
    begin: usize,
    end: usize,
}

pub fn run(expression: &String, mut source: impl Iterator<Item = u8>) -> Vec<MatchResult> {
    let mut matches = Vec::new();

    if let Some(mut window) = build_first_window(expression, &mut source) {
        let mut begin = 0;

        loop {
            if expression_matches_window(&expression, &window) {
                matches.push(MatchResult {
                    begin,
                    end: begin + expression.len() - 1,
                })
            }
            match source.next() {
                Some(value) => {
                    window.pop_front();
                    window.push_back(value);
                    begin += 1;
                }
                None => break,
            }
        }
    }

    matches
}

fn expression_matches_window(expression: &String, window: &VecDeque<u8>) -> bool {
    window
        .iter()
        .zip(expression.as_bytes().iter())
        .all(|(a, b)| *a == *b)
}

fn build_first_window(
    expression: &String,
    source: &mut impl Iterator<Item = u8>,
) -> Option<VecDeque<u8>> {
    let expression_len = expression.as_bytes().len();
    let mut window = VecDeque::new();

    for _ in 0..expression_len {
        match source.next() {
            Some(byte) => window.push_back(byte),
            None => return None,
        };
    }

    return Some(window);
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
