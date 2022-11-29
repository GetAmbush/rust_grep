mod args;
mod file_reader;

use args::Arguments;
use std::fs::File;

fn main() {
    let std_args = std::env::args();
    let args = Arguments::from(std_args);
    dbg!("{:?}", &args);

    let file_handle = File::open(&args.file).unwrap();
    let file_contents = file_reader::read_file(file_handle);

    dbg!("{:?}", file_contents);
}
