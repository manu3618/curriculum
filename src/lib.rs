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

#[derive(Serialize, Deserialize, Debug)]
struct Curriculum {
    #[serde(rename = "personal data")]
    personal_data: PersonalData,
    education: Vec<CVEntry>,
    experiences: Vec<EntryDescription>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PersonalData {
    name: String,
    title: Option<String>,
    #[serde(default)]
    mobile: Vec<String>,
    #[serde(default)]
    email: Vec<String>,
    github: Option<String>,
    gitlab: Option<String>,
    twitter: Option<String>,
    linkedin: Option<String>,

    #[serde(default)]
    /// [(name, url), ]
    webpage: Vec<(String, String)>,
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

impl PersonalData {
    fn to_latex(&self) -> String {
        let mut lines = Vec::new();
        lines.push("% personal data".into());
        let names = &self.name.split(' ').collect::<Vec<_>>();
        let first_name = names[0];
        lines.push(format!("\\firstname{{\\LARGE {first_name}}}"));
        if let Some(last_name) = names.get(1) {
            lines.push(format!("\\familyname{{\\LARGE {last_name}}}"));
        }
        if let Some(title) = &self.title {
            lines.push(format!("\\title{{{}}}", title));
        }
        for t in &self.mobile {
            lines.push(format!("\\mobile{{{}}}", t));
        }
        for e in &self.email {
            lines.push(format!("\\email{{{}}}", e));
        }
        // socials
        if let Some(e) = &self.github {
            lines.push(format!("\\social[github]{{{e}}}"))
        }
        if let Some(e) = &self.gitlab {
            lines.push(format!("\\social[gitlab]{{{e}}}"))
        }
        if let Some(e) = &self.linkedin {
            lines.push(format!("\\social[linkedin]{{{e}}}"))
        }
        if let Some(e) = &self.twitter {
            lines.push(format!("\\social[twitter]{{{e}}}"))
        }
        for (n, u) in &self.webpage {
            lines.push(format!("\\extrainfo{{\\homepagesymbol {n} \\url{{{u}}}}}"));
        }
        lines.join("\n")
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

    #[test]
    fn read_personal_data() {
        let data = r#"
        {
            "name": "Jessica",
            "title": "Environmental manager",
            "mobile": ["+32 56 19 01"]
        }"#;
        let personal_data: PersonalData = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn write_incomplete_personal_data() {
        let data = r#"
        {
            "name": "Jessica Meyer",
            "title": "Environmental manager",
            "mobile": ["+32 56 19 01", "+32 56 19 04"]
        }"#;
        let personal_data: PersonalData = serde_json::from_str(&data).unwrap();
        let result = personal_data.to_latex();

        eprintln!("{result}");
        assert!(false);
    }

    #[test]
    fn write_personal_data() {
        let data = r#"
        {
            "name": "Jessica Meyer",
            "title": "Environmental manager",
            "mobile": ["+32 56 19 01", "+32 56 19 04"],
            "email": ["nom@example.com", "nom@example.org"],
            "github": "example",
            "webpage": [["example", "www.example.com"]]
        }"#;
        let personal_data: PersonalData = serde_json::from_str(&data).unwrap();
        let result = personal_data.to_latex();

        eprintln!("{result}");
        assert!(false);
    }
}
