use std::io::{self, BufRead};

pub fn read_file(file_handle: std::fs::File) -> Vec<String> {
    let buf_reader = io::BufReader::new(file_handle).lines();
    let mut file_lines = Vec::new();

    for line in buf_reader {
        file_lines.push(line.unwrap())
    }

    file_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_to_lines() {
        let file_handle = std::fs::File::open("example.txt").expect("failed to read file");
        let lines_read = read_file(file_handle);

        assert_eq!(lines_read.len(), 4);
    }
}
