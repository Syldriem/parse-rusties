use clap::Parser;

fn main() {
    let opts = rust::options::Opts::parse();
    println!("{:?}", opts)
}
