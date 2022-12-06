mod args;
mod errors;
mod file_reader;
mod grep;

use args::Arguments;
use errors::Error;

fn main() -> Result<(), Error> {
    let std_args: Vec<String> = std::env::args().collect();
    let args = Arguments::try_from(std_args)?;
    dbg!("{:?}", &args);

    let file_contents = grep(args)?;
    dbg!("{:?}", file_contents);

    Ok(())
}

fn grep(args: Arguments) -> Result<Vec<(usize, String)>, Error> {
    let file_contents = file_reader::read_file(&args.file_path)?
        .into_iter()
        .enumerate()
        .filter(|(_index, string)| string.contains(&args.regex))
        .collect::<Vec<_>>();

    Ok(file_contents)
}
