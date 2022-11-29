mod args;
mod errors;
mod file_reader;

use args::Arguments;
use errors::Error;
use std::fs::File;

fn main() -> Result<(), Error> {
    let std_args = std::env::args();
    let args = Arguments::try_from(std_args)?;
    dbg!("{:?}", &args);

    let file_handle = File::open(&args.file_path).unwrap();
    let file_contents = file_reader::read_file(file_handle);

    dbg!("{:?}", file_contents);

    Ok(())
}
