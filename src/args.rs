#[derive(Debug)]
pub struct Arguments {
    executable_name: String,
    file: String,
    regex: String,
}

impl Arguments {
    pub fn from_std(mut args: impl Iterator<Item = String>) -> Self {
        let executable_name = match args.next() {
            Some(text) => text,
            None => panic!("sdiofsdf"),
        };

        let regex = match args.next() {
            Some(text) => text,
            None => panic!("sdiofsdf"),
        };

        let file = match args.next() {
            Some(text) => text,
            None => panic!("sdiofsdf"),
        };

        Self {
            regex,
            file,
            executable_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_std_arguments() {
        let executable_name_string = "executable_name".to_string();
        let regex_string = "regex".to_string();
        let file_string = "file.txt".to_string();

        let std_args = vec![
            executable_name_string.clone(),
            regex_string.clone(),
            file_string.clone(),
        ];
        let arguments = Arguments::from_std(std_args.into_iter());

        assert_eq!(arguments.executable_name, executable_name_string);
        assert_eq!(arguments.file, file_string);
        assert_eq!(arguments.regex, regex_string);
    }
}
