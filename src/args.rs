use crate::errors::{Error, MissingArguments};

#[derive(Debug)]
pub struct Arguments {
    executable_name: String,
    pub file_path: String,
    regex: String,
}

impl Arguments {
    pub fn try_from(mut args: impl Iterator<Item = String>) -> Result<Self, Error> {
        let executable_name = args
            .next()
            .ok_or_else(|| Error::MissingArguments(MissingArguments::ExecutableName))?;

        let regex = args
            .next()
            .ok_or_else(|| Error::MissingArguments(MissingArguments::FilePath))?;

        let file_path = args
            .next()
            .ok_or_else(|| Error::MissingArguments(MissingArguments::Regex))?;

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
        let arguments = Arguments::try_from(std_args.into_iter())
            .expect("could not convert arguments vector to parsed structure");

        assert_eq!(arguments.executable_name, executable_name_string);
        assert_eq!(arguments.file_path, file_string);
        assert_eq!(arguments.regex, regex_string);
    }
}
