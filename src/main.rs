mod args;

fn main() {
    let std_args = std::env::args();
    let args = args::Arguments::from_std(std_args);
    println!("{:?}", args);

    // println!("Hello, world!");
}
