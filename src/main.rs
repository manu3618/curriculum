use clap::Parser;
use std::fs;
use std::path::Path;
// use tectonic; // TODO use an optional dependency

#[derive(Parser, Debug)]
#[command(author, about)]
struct Args {
    /// input filename
    input: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.input);
    let out_tex = path.with_extension("tex");
    let in_ext = path.extension().unwrap();
    dbg!(&path);
    dbg!(&outname);
    dbg!(&in_ext);
    let tex_data: String = todo!();
    fs::write(path.with_extension("tex"), tex_data);

    // TODO: use an optional dependency
    // let pdf_data: Vec<u8> = tectonic::latex_to_pdf(tex_data).expect("processing failed");
    // println!("Output PDF size is {} bytes", pdf_data.len());
    // fs::write(path.with_extension("pdf"), pdf_data);
}
