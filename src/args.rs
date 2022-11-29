use crate::errors::Error;

#[derive(Debug)]
pub struct Arguments {
    pub executable_name: String,
    pub file_path: String,
    pub regex: String,
}

impl Arguments {
    pub fn try_from(args: Vec<String>) -> Result<Self, Error> {
        if args.len() != 3 {
            return Err(Error::InvalidNumberOfArguments);
        }

        let executable_name = args[0].clone();
        let regex = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self {
            executable_name,
            regex,
            file_path,
        })
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
        let arguments = Arguments::try_from(std_args)
            .expect("could not convert arguments vector to parsed structure");

        assert_eq!(arguments.executable_name, executable_name_string);
        assert_eq!(arguments.file_path, file_string);
        assert_eq!(arguments.regex, regex_string);
    }

    #[test]
    fn errors_when_does_not_have_correct_number_of_arguments() {
        let std_args = vec![];
        let result = Arguments::try_from(std_args);
        assert!(matches!(result, Err(Error::InvalidNumberOfArguments)))
    }
}
