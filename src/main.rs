mod args;
mod errors;
mod file_reader;

use args::Arguments;
use errors::Error;

fn main() -> Result<(), Error> {
    let std_args: Vec<String> = std::env::args().collect();
    let args = Arguments::try_from(std_args)?;
    dbg!("{:?}", &args);

    let file_contents = file_reader::read_file(&args.file_path)?;
    dbg!("{:?}", file_contents);

    Ok(())
}
