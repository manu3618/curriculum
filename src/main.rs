use anyhow::Result;
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

fn main() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.input);
    let in_ext = path.extension().unwrap();
    dbg!(&path);
    dbg!(&in_ext);

    let content = fs::read_to_string(path)?;
    let cv: curriculum::Curriculum = serde_json::from_str(&content)?;
    dbg!(&cv);
    let tex_data = cv.to_latex()?;
    let tex_path = path.with_extension("tex");
    println!("wrinting {}", tex_path.display());
    let _ = fs::write(tex_path, tex_data);

    // TODO: use an optional dependency
    // let pdf_data: Vec<u8> = tectonic::latex_to_pdf(tex_data).expect("processing failed");
    // println!("Output PDF size is {} bytes", pdf_data.len());
    // fs::write(path.with_extension("pdf"), pdf_data);
    Ok(())
}
