use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::Path;

#[cfg(feature = "pdf")]
use tectonic;

#[derive(Parser, Debug)]
#[command(author, about)]
struct Args {
    /// input filename
    input: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.input);

    let content = fs::read_to_string(path)?;
    let cv: curriculum::Curriculum = serde_json::from_str(&content)?;
    // dbg!(&cv);
    let tex_data = cv.to_latex()?;
    let tex_path = path.with_extension("tex");
    println!("writing {}", tex_path.display());
    let _ = fs::write(&tex_path, tex_data);

    #[cfg(feature = "pdf")]
    cv.to_pdf(Some(&tex_path))?;

    Ok(())
}
