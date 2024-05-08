use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Serialize, Deserialize, Debug)]
struct CVEntry {
    // beginning: Date,
    // end: Option<Date>,
    /// degree or title
    #[serde(default)]
    degree: String,
    /// insititution or company
    #[serde(default)]
    institution: String,
    #[serde(default)]
    city: Option<String>,
    #[serde(default)]
    grade: Option<String>,
    #[serde(default)]
    description: Option<EntryDescription>,
}

#[derive(Serialize, Deserialize, Debug)]
struct EntryDescription {
    context: String,
    /// technologies
    /// programming language
    programming: Vec<String>,
    /// version control
    version: Vec<String>,
    database: Vec<String>,
    cloud: Vec<String>,
    /// CI/CD
    ci: Vec<String>,
    other: Vec<String>,
}

#[derive(Debug)]
struct Curriculum {
    education: Vec<CVEntry>,
    experiences: Vec<EntryDescription>,
}

#[derive(Debug)]
struct List(Vec<String>);

fn make_first_page() -> String {
    todo!()
}

impl CVEntry {
    fn to_latex(&self) -> String {
        todo!()
    }
}

impl List {
    fn to_latex(&self) -> String {
        format!(
            r#"\begin{{itemize}}
        {}
        \end{{itemize}}"#,
            &self
                .0
                .iter()
                .map(|elt| format!("\\item {}", elt))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

mod tests {
    use super::*;

    #[test]
    fn read_incomplete_entry() {
        let entry = r#"
        {
            "degree": "highest degree",
            "institution": "prestigious institution",
            "city": "Brussels, Belgium"
        }
        "#;
        let entry: CVEntry = serde_json::from_str(&entry).unwrap();
    }
}
