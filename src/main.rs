use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, about)]
struct Args {
    /// input filename
    input: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.input);
    let outname = path.with_extension("tex");
    let in_ext = path.extension().unwrap();
    dbg!(&path);
    dbg!(&outname);
    dbg!(&in_ext);
}
