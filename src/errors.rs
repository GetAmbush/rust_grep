#[derive(Debug)]
pub enum Error {
    MissingArguments(MissingArguments),
}

#[derive(Debug)]
pub enum MissingArguments {
    ExecutableName,
    FilePath,
    Regex,
}
