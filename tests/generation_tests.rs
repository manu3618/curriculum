use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[test]
fn tex_generation() {
    let mut treated = 0;
    for entry in Path::new("./tests").read_dir().expect("read_dir failed?") {
        if entry.as_ref().unwrap().path().extension().unwrap() != "json" {
            eprintln!("ignoring {:?}", entry.as_ref());
            continue;
        }
        let file = File::open(entry.as_ref().unwrap().path()).expect("file exists");
        let mut out_path = Path::new("/tmp").join(entry.as_ref().unwrap().file_name());
        out_path.set_extension("tex");
        let reader = BufReader::new(file);
        let cv: curriculum::Curriculum =
            serde_json::from_reader(reader).expect("that's what we test");
        let tex_data = cv.to_latex().unwrap();
        assert!(tex_data.len() > 0);
        eprintln!("writing to {}", out_path.display());
        let _ = fs::write(out_path.clone(), tex_data);
        assert!(out_path.exists());
        assert!(out_path.is_file());

        treated += 1;
    }
    assert!(treated > 0);
}

#[test]
fn json_generation() -> Result<()> {
    let mut treated = 0;
    for entry in Path::new("./tests").read_dir().expect("read_dir failed?") {
        if entry.as_ref().unwrap().path().extension().unwrap() != "json" {
            eprintln!("ignoring {:?}", entry.as_ref());
            continue;
        }
        let out_path = Path::new("/tmp").join(entry.as_ref().unwrap().file_name());
        let content = fs::read_to_string(entry.unwrap().path())?;
        let cv: curriculum::Curriculum = serde_json::from_str(&content)?;

        let cv_j = serde_json::to_string(&cv)?;
        eprintln!("writing to {}", out_path.display());
        fs::write(out_path.clone(), cv_j)?;
        assert!(out_path.exists());
        assert!(out_path.is_file());
        treated += 1;
    }
    assert!(treated > 0);
    Ok(())
}
