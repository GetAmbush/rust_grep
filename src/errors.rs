#[derive(Debug)]
pub enum Error {
    InvalidNumberOfArguments,
    CannotOpenFile(std::io::Error),
}
