mod args;
mod file_reader;

use args::Arguments;

fn main() {
    let std_args = std::env::args();
    let args = Arguments::from(std_args);
    dbg!("{:?}", args);

    // println!("Hello, world!");
}
