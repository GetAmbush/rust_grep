use std::fs::File;
use std::io::{self, BufRead};

use crate::errors::Error;

pub fn read_file(file_path: &str) -> Result<Vec<String>, Error> {
    let file_handle = File::open(file_path).map_err(Error::CannotOpenFile)?;
    let buf_reader = io::BufReader::new(file_handle).lines();
    let mut file_lines = Vec::new();

    for line in buf_reader {
        file_lines.push(line.unwrap())
    }

    Ok(file_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_to_lines() {
        let lines_read = read_file("poem.txt").unwrap();
        assert_eq!(lines_read.len(), 9);
    }

    #[test]
    fn fail_to_open_file() {
        let result = read_file("i_dont_exist.txt");
        assert!(matches!(result, Err(Error::CannotOpenFile(_))))
    }
}
