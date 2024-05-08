use curriculum;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[test]
fn tex_generation() {
    for entry in Path::new(".").read_dir().expect("read_dir failed?") {
        if !entry.as_ref().unwrap().path().ends_with("json") {
            continue;
        }
        let file = File::open(entry.unwrap().path()).expect("file exists");
        let reader = BufReader::new(file);
        let cv: curriculum::Curriculum =
            serde_json::from_reader(reader).expect("that's what we test");
        let tex_data = cv.to_latex().unwrap();
        assert!(tex_data.len() > 0);
    }
}
