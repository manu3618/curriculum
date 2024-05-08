use anyhow::Result;
use chrono::{DateTime, Duration, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

static DATA_DIR: &str = "data";

#[derive(Serialize, Deserialize, Debug, Default)]
struct CVEntry {
    #[serde(default)]
    #[serde(with = "cv_date")]
    beginning: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(with = "cv_date")]
    end: Option<DateTime<Utc>>,
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

#[derive(Serialize, Deserialize, Debug, Default)]
struct EntryDescription {
    #[serde(default)]
    context: String,
    /// technologies
    /// programming language
    #[serde(default)]
    programming: Vec<String>,
    /// version control
    #[serde(default)]
    version: Vec<String>,
    #[serde(default)]
    database: Vec<String>,
    #[serde(default)]
    cloud: Vec<String>,
    /// CI/CD
    #[serde(default)]
    ci: Vec<String>,
    #[serde(default)]
    other: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Curriculum {
    #[serde(rename = "personal data")]
    personal_data: PersonalData,
    education: Vec<CVEntry>,
    experiences: Vec<EntryDescription>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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

#[derive(Debug, PartialEq, Eq)]
struct CVDuration {
    year: u32,
    month: u32,
}

impl CVEntry {
    fn to_latex(&self) -> String {
        todo!()
    }

    /// get skills
    /// {category:Vec<skills>}
    fn extract_skills(&self) -> HashMap<&str, Vec<String>> {
        let mut skills = HashMap::new();
        if let Some(desc) = &self.description {
            skills.insert("programming languages", desc.programming.clone());
            skills.insert("version control", desc.version.clone());
            skills.insert("databases", desc.database.clone());
            skills.insert("cloud computing", desc.cloud.clone());
            skills.insert("CI/CD", desc.ci.clone());
            skills.insert("other", desc.other.clone());
        }
        skills.retain(|_, v| !&v.is_empty());
        skills
    }

    /// return duration of this entry
    fn duration(&self) -> Option<Duration> {
        if let Some(b) = &self.beginning {
            if let Some(e) = &self.end {
                Some(*e - b)
            } else {
                Some(Utc::now() - b)
            }
        } else {
            None
        }
    }

    fn cv_duration(&self) -> Option<CVDuration> {
        if let Some(duration) = &self.duration() {
            let duration: u32 = duration.num_days() as u32;
            let year = duration / 365;
            let remaining_days = duration % 365;
            let month = (remaining_days + 15) / 30;
            Some(CVDuration { year, month })
        } else {
            None
        }
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

impl EntryDescription {
    fn to_latex(&self) -> String {
        todo!()
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

impl Curriculum {
    /// Generate the LaTeX corresponding to the whole document
    pub fn to_latex(&self) -> Result<String> {
        let mut output = Vec::new();
        let preamb = fs::read(Path::new(DATA_DIR).join("preambule.tex"))?;
        output.push(String::from_utf8(preamb)?);

        output.push("\\end{document}".into());
        Ok(output.join("\n"))
    }
    /// Get skills from entries
    /// {category: {skill: duration}}
    fn get_skills(&self) -> HashMap<String, HashMap<String, String>> {
        todo!()
    }
}

mod cv_date {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};
    const FORMAT: &'static str = "%Y-%m";

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(date) = date {
            let s = format!("{}", date.format(FORMAT));
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            let date: Vec<_> = s.split('-').collect();
            let date: DateTime<Utc> = Utc
                .with_ymd_and_hms(
                    str::parse(date[0]).unwrap(),
                    str::parse(date[1]).unwrap(),
                    1,
                    0,
                    0,
                    0,
                )
                .unwrap();
            Ok(Some(date))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
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
    }

    #[test]
    fn decription_tex() {
        let data = r#"
        {
            "context": "some super context",
            "ci": ["git", "gitlab"]
        }
        "#;
        let entry: EntryDescription = serde_json::from_str(&data).unwrap();
        let result = entry.to_latex();
    }

    #[test]
    fn entry_extract_skills() {
        let data = r#"
        {
            "beginning": "2023-11",
            "city": "Brussels",
            "description":
                 {
                    "context": "some super context",
                    "ci": ["git", "gitlab"]
                }
        }
        "#;
        let entry: CVEntry = serde_json::from_str(&data).unwrap();
        let skills = entry.extract_skills();
        assert!(skills.keys().collect::<Vec<_>>().contains(&&"CI/CD"));
    }

    #[test]
    fn entry_duration() {
        let data = r#"
        {
            "beginning": "2023-10",
            "end" : "2023-12",
            "city": "Brussels",
            "description":
                 {
                    "context": "some super context",
                    "ci": ["git", "gitlab"]
                }
        }
        "#;
        let entry: CVEntry = serde_json::from_str(&data).unwrap();
        let duration = entry.cv_duration();
        assert_eq!(duration, Some(CVDuration { year: 0, month: 2 }));
    }

    #[test]
    fn entry_duration_long() {
        let data = r#"
        {
            "beginning": "2013-10",
            "end" : "2023-12",
            "city": "Brussels",
            "description":
                 {
                    "context": "some super context",
                    "ci": ["git", "gitlab"]
                }
        }
        "#;
        let entry: CVEntry = serde_json::from_str(&data).unwrap();
        let duration = entry.cv_duration();
        assert_eq!(duration, Some(CVDuration { year: 10, month: 2 }));
    }

    #[test]
    fn entry_duration_null() {
        let data = r#"
        {
            "beginning": "2023-10",
            "end" : "2023-10",
            "city": "Brussels",
            "description":
                 {
                    "context": "some super context",
                    "ci": ["git", "gitlab"]
                }
        }
        "#;
        let entry: CVEntry = serde_json::from_str(&data).unwrap();
        let duration = entry.cv_duration();
        assert_eq!(duration, Some(CVDuration { year: 0, month: 0 }));
    }

    #[test]
    fn entry_duration_none() {
        let data = r#"
        {
            "city": "Brussels",
            "description":
                 {
                    "context": "some super context",
                    "ci": ["git", "gitlab"]
                }
        }
        "#;
        let entry: CVEntry = serde_json::from_str(&data).unwrap();
        let duration = entry.cv_duration();
        assert_eq!(duration, None);
    }

    #[test]
    fn entry_duration_noend() {
        let data = r#"
        {
            "beginning": "2023-10",
            "city": "Brussels",
            "description":
                 {
                    "context": "some super context",
                    "ci": ["git", "gitlab"]
                }
        }
        "#;
        let entry: CVEntry = serde_json::from_str(&data).unwrap();
        let duration = entry.cv_duration().unwrap();
        assert!(duration.month + duration.year > 0);
    }
}
