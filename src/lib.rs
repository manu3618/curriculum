use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Add;

static PREAMBULE: &str = include_str!("../data/preambule.tex");

/// list of ordered skill categories
const SKILL_CATEGORIES: &[&str] = &[
    "prorgamming languages",
    "version control",
    "database",
    "cloud computing",
    "CI/CD",
    "other",
];

#[derive(Debug)]
enum Industry {
    Energy,
    Telecommunications,
    Health,
    Insurance,
    Automotive,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct CVEntry {
    #[serde(default)]
    #[serde(with = "cv_date")]
    beginning: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(with = "cv_date")]
    end: Option<DateTime<Utc>>,
    /// degree or title or name
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
    #[serde(default)]
    subentries: Vec<CVEntry>,
}

impl CVEntry {
    /// Produce corresponding LaTeX
    fn to_latex(&self) -> String {
        let mut descr = match &self.description {
            Some(d) => d.to_latex(),
            None => "".into(),
        };
        let max_date_len = &self.subentries.iter().map(|e| e.get_dates().len()).max();
        for subentry in &self.subentries {
            descr.push('\n');
            if let Some(margin) = max_date_len {
                descr.push_str(&format!("\\hspace*{{-{}ex}}", 21.5 - *margin as f32));
            }
            descr.push_str(&subentry.to_latex());
        }
        format!(
            "\\cventry{{{}}}{{{}}}{{{}}}{{{}}}{{{}}}{{%\n{}%\n}}",
            &self.get_dates(),
            &self.degree, // title
            &self.institution,
            &self.city.clone().unwrap_or("".into()),
            &self.grade.clone().unwrap_or("".into()),
            descr.trim(),
        )
    }

    fn get_dates(&self) -> String {
        let mut dates: Vec<String> = Vec::new();
        if let Some(b) = self.beginning {
            dates.push(format!("{}", b.format("%Y")))
        };
        if let Some(e) = self.end {
            dates.push(format!("{}", e.format("%Y")))
        };
        dates.join("--")
    }

    /// get skills
    /// {category: [skills]}
    fn extract_skills(&self) -> HashMap<&str, Vec<String>> {
        if let Some(desc) = &self.description {
            desc.extract_skills()
        } else {
            HashMap::new()
        }
    }

    /// get skills with duration
    /// {category: {skill: duration}}
    fn extract_skills_duration(&self) -> HashMap<&str, HashMap<String, Duration>> {
        if let Some(desc) = &self.description {
            (desc.extract_skills().iter().map(|(&category, skills)| {
                (
                    category,
                    skills
                        .iter()
                        .map(|s| (s.clone(), self.duration().unwrap_or(Duration::zero())))
                        .collect::<HashMap<_, _>>(),
                )
            }))
            .collect::<HashMap<_, _>>()
        } else {
            HashMap::new()
        }
    }

    /// get skills with duration, including subentries
    /// {category: {skill: duration}}
    fn extract_subentries_skills(&self) -> HashMap<&str, HashMap<String, Duration>> {
        let skills = self.extract_skills_duration();
        self.subentries.iter().fold(skills, |mut acc, entry| {
            add_skillsets(&mut acc, entry.extract_subentries_skills());
            acc
        })
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

/// Add skillset from other to acc
fn add_skillsets<'a, I, S>(
    acc: &mut HashMap<&'a str, HashMap<String, Duration>>,
    other: HashMap<&'a str, I>,
) where
    I: IntoIterator<Item = (S, Duration)>,
    S: ToString,
    String: From<S>,
{
    for (category, skills) in other {
        let cat = acc.entry(category).or_default();
        for (skill, duration) in skills {
            cat.entry(skill.into())
                .and_modify(|d| *d = *d + duration)
                .or_insert(duration);
        }
    }
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

impl EntryDescription {
    fn extract_skills(&self) -> HashMap<&str, Vec<String>> {
        let mut skills = HashMap::new();
        skills.insert("programming languages", self.programming.clone());
        skills.insert("version control", self.version.clone());
        skills.insert("database", self.database.clone());
        skills.insert("cloud computing", self.cloud.clone());
        skills.insert("CI/CD", self.ci.clone());
        skills.insert("other", self.other.clone());
        skills.retain(|_, v| !&v.is_empty());
        skills
    }

    fn to_latex(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push("%".into());
        lines.push((&self.context).into());
        let skills = &self.extract_skills();
        if !skills.is_empty() {
            lines.push("\\begin{description}".into());
            for name in SKILL_CATEGORIES {
                if let Some(list) = skills.get(name) {
                    lines.push(format!("\\item [{}] {}", name, list.join(", ")))
                }
            }
            lines.push("\\end{description}\n".into());
        }
        lines.join("\n")
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Curriculum {
    #[serde(rename = "personal data")]
    personal_data: PersonalData,
    education: Vec<CVEntry>,
    experiences: Vec<CVEntry>,
    #[serde(default)]
    languages: Vec<CVLanguage>,
}

impl Curriculum {
    /// Generate the LaTeX corresponding to the whole document
    pub fn to_latex(&self) -> Result<String> {
        let mut output = Vec::new();
        let preamb = PREAMBULE.into();
        output.push(String::from_utf8(preamb)?);

        // TODO replace with first page
        // TODO add skills
        output.push(self.personal_data.to_latex());
        output.push("\n\\begin{document}\n".into());
        output.push("\\maketitle".into());

        output.push("\\section{Education}".into());
        for edu in &self.education {
            output.push(edu.to_latex());
            output.push("\n".into());
        }

        output.push("\\section{Proffesional experience}".into());
        for experience in &self.experiences {
            output.push(experience.to_latex());
            output.push("\n".into());
        }

        output.push("\\section{Languages}".into());
        for language in &self.languages {
            output.push(language.to_latex());
            output.push("\n".into());
        }


        output.push("\\end{document}".into());
        Ok(output.join("\n"))
    }

    #[cfg(feature = "pdf")]
    /// Generate pdf
    /// if path is not None, write file
    /// return the content of the pdf file
    pub fn to_pdf(&self, path: Option<&Path>) -> Result<Vec<u8>> {
        let tex_data = &self.to_latex()?;
        if let Some(tex_path) = path {
            let tex_path = tex_path.with_extension("tex");
            println!(
                "writing to {}",
                tex_path.to_str().expect("path should be valid")
            );
            fs::write(tex_path, tex_data)?;
        }
        let pdf_data: Vec<u8> = tectonic::latex_to_pdf(tex_data).unwrap();
        if let Some(pdf_path) = path {
            let pdf_path = pdf_path.with_extension("pdf");
            println!(
                "writing to {}",
                pdf_path.to_str().expect("path should be valid")
            );
            fs::write(pdf_path, pdf_data.clone())?;
        }
        Ok(pdf_data)
    }

    /// Get skills from entries
    /// {category: {skill: duration}}
    fn get_skills(&self) -> HashMap<&str, HashMap<String, CVDuration>> {
        let mut ret_skills = HashMap::new();
        for xp in &self.experiences {
            let duration = &xp.cv_duration().unwrap_or_default();
            let entry_skills = xp.extract_skills();
            for (categ, ref skills) in entry_skills {
                let ret_categ: &mut HashMap<String, _> = ret_skills.entry(categ).or_default();
                for skill in skills {
                    let s: &mut CVDuration = ret_categ.entry(skill.clone()).or_default();
                    *s = s.clone() + duration.clone();
                }
            }
        }
        ret_skills
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CVEmail {
    #[serde(default)]
    name: Option<String>,
    mail: String,
}

impl CVEmail {
    fn to_latex(&self) -> String {
        let link = format!("\\href{{mailto:{}}}{{{}}}", self.mail, self.mail);
        if let Some(name) = &self.name {
            format!("{}: {}", name, link)
        } else {
            link
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct PersonalData {
    name: String,
    title: Option<String>,
    #[serde(default)]
    mobile: Vec<String>,
    #[serde(default)]
    email: Vec<CVEmail>,
    github: Option<String>,
    gitlab: Option<String>,
    twitter: Option<String>,
    linkedin: Option<String>,

    #[serde(default)]
    /// [(name, url), ]
    webpage: Vec<(String, String)>,
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
        } else {
            lines.push("\\familyname{{}}".into());
        }
        if let Some(title) = &self.title {
            lines.push(format!("\\title{{{}}}", title));
        }
        for t in &self.mobile {
            lines.push(format!("\\mobile{{{}}}", t));
        }
        for e in &self.email {
            lines.push(format!("\\email{{{}}}", e.mail));
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

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
struct CVLanguage {
    language: String,
    #[serde(default)]
    level: String,
    #[serde(default)]
    comment: String,
}

impl CVLanguage {
    fn to_latex(&self) -> String {
        format!(
            "\\cvlanguage{{{}}}{{{}}}{{{}}}",
            self.language, self.level, self.comment
        )
    }
}
#[derive(Debug)]
struct List(Vec<String>);

/// create the first page
///
/// The fisrt page should sum up the resume, including
/// * technical knowledge (ventilated by experience?)
/// * functional knowledge
/// * industry knowledge (in which industry your work in)
fn make_first_page() -> String {
    todo!()
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CVDuration {
    pub year: u32,
    pub month: u32,
}

impl CVDuration {
    /// Round to nearest year number if duration is more than 10 months
    /// ```
    /// use curriculum::CVDuration;
    ///
    /// let d = CVDuration {year: 3, month: 3};
    /// assert_eq!(d.round(), CVDuration {year: 3, month: 0});
    ///
    /// let d = CVDuration {year: 3, month: 9};
    /// assert_eq!(d.round(), CVDuration {year: 4 , month: 0});
    ///
    /// let d = CVDuration {year: 3, month: 7};
    /// assert_eq!(d.round(), CVDuration {year: 4 , month: 0});
    ///
    /// let d = CVDuration {year: 3, month: 6};
    /// assert_eq!(d.round(), CVDuration {year: 4 , month: 0});
    ///
    /// let d = CVDuration {year: 3, month: 5};
    /// assert_eq!(d.round(), CVDuration {year: 3 , month: 0});
    ///
    /// let d = CVDuration {year: 0, month: 11};
    /// assert_eq!(d.round(), CVDuration {year: 1, month: 0});
    ///
    /// let d = CVDuration {year: 0, month: 10};
    /// assert_eq!(d.round(), CVDuration {year: 0, month: 10});
    /// ```
    pub fn round(&self) -> Self {
        if self.year == 0 && self.month <= 10 {
            Self { ..*self }
        } else {
            Self {
                year: (self.year * 12 + self.month + 6) / 12,
                month: 0,
            }
        }
    }
}

impl Add for CVDuration {
    type Output = Self;

    /// Add durations
    /// ```
    /// use curriculum::CVDuration;
    ///
    /// let d1 = CVDuration {year: 1, month: 9};
    /// let d2 = CVDuration {year: 0, month: 8};
    /// let d3 = CVDuration {year: 2, month: 0};
    ///
    /// assert_eq!(d1.clone() + d2.clone(), CVDuration{ year:2, month: 5 });
    /// assert_eq!(d1 + d3.clone(), CVDuration{ year:3, month: 9 });
    /// assert_eq!(d2 + d3, CVDuration{ year:2, month: 8 });
    /// ```
    fn add(self, other: Self) -> Self {
        let m = self.month + other.month;
        let y = self.year + other.year;
        if m < 12 {
            Self { year: y, month: m }
        } else {
            Self {
                year: y + (m / 12),
                month: m % 12,
            }
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

mod cv_date {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};
    const FORMAT: &str = "%Y-%m";

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
    use regex::Regex;

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
            "email": [{"mail": "nom@example.com"}, {"mail": "nom@example.org"}],
            "github": "example",
            "webpage": [["example", "www.example.com"]]
        }"#;
        let personal_data: PersonalData = serde_json::from_str(&data).unwrap();
        let result = personal_data.to_latex();
    }

    #[test]
    fn description_tex() {
        let data = r#"
        {
            "context": "some super context",
            "ci": ["git", "gitlab"]
        }
        "#;
        let entry: EntryDescription = serde_json::from_str(&data).unwrap();
        let result = entry.to_latex();
        // assert!(false);
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

    #[test]
    fn get_cv_skills() {
        let data = r#"
        {
            "personal data": {
                "name": "Jessica",
                "title": "Environmental manager",
                "mobile": ["+32 56 19 01"]
                },
            "education": [],
            "experiences": [
                {
                    "beginning": "2022-10",
                    "end": "2023-11",
                    "city": "Brussels",
                    "description":
                         {
                            "context": "some super context",
                            "ci": ["git", "gitlab"]
                        }
                },
                {
                    "beginning": "2022-09",
                    "end": "2023-07",
                    "city": "Brussels",
                    "description":
                         {
                            "context": "some super context",
                            "ci": ["git", "gitlab"],
                            "cloud": ["azure"]
                        }
                }

            ]
        }
        "#;
        let cv: Curriculum = serde_json::from_str(&data).unwrap();
        let s = cv.get_skills();
        assert_eq!(s["CI/CD"]["git"].clone(), CVDuration { year: 1, month: 11 });
        assert_eq!(
            s["cloud computing"]["azure"],
            CVDuration { year: 0, month: 10 }
        );
    }

    #[test]
    fn write_email() {
        let data = r#"
        [
            {
                "name": "John",
                "mail": "j@doe.org"
            },
            {
                "mail": "blah@example.com"
            }
        ]
        "#;
        let emails: Vec<CVEmail> = serde_json::from_str(&data).unwrap();
        let tex: Vec<String> = emails.iter().map(|e| e.to_latex()).collect();
        assert!(tex[0].contains("mailto:"));
        assert!(tex[0].contains("John"));
        assert!(tex[1].contains("mailto:"));
    }

    #[test]
    fn skills_accumulator() {
        let mut skills0 = HashMap::new();
        let skills1 = HashMap::from([
            (
                "languages",
                HashMap::from([
                    ("english", Duration::days(20)),
                    ("spanish", Duration::days(30)),
                    ("french", Duration::days(1)),
                ]),
            ),
            ("management", HashMap::from([("jira", Duration::zero())])),
        ]);

        add_skillsets(&mut skills0, skills1);
        assert_eq!(
            skills0.get("languages").and_then(|s| s.get("english")),
            Some(Duration::days(20)).as_ref()
        );
        assert_eq!(
            skills0.get("management").and_then(|s| s.get("jira")),
            Some(Duration::zero()).as_ref()
        );

        let skills2 = HashMap::from([
            (
                "languages",
                HashMap::from([
                    ("arabic", Duration::days(3)),
                    ("french", Duration::days(10)),
                ]),
            ),
            ("driving", HashMap::from([("cars", Duration::zero())])),
        ]);
        add_skillsets(&mut skills0, skills2);
        assert_eq!(
            skills0.get("languages").and_then(|s| s.get("english")),
            Some(Duration::days(20)).as_ref()
        );
        assert_eq!(
            skills0.get("languages").and_then(|s| s.get("spanish")),
            Some(Duration::days(30)).as_ref()
        );
        assert_eq!(
            skills0.get("languages").and_then(|s| s.get("french")),
            Some(Duration::days(11)).as_ref()
        );
        assert_eq!(
            skills0.get("languages").and_then(|s| s.get("arabic")),
            Some(Duration::days(3)).as_ref()
        );
        assert_eq!(
            skills0.get("management").and_then(|s| s.get("jira")),
            Some(Duration::zero()).as_ref()
        );
        assert_eq!(
            skills0.get("driving").and_then(|s| s.get("cars")),
            Some(Duration::zero()).as_ref()
        );
    }

    #[test]
    fn subentries() {
        let data = r#"
        {
            "beginning": "1977-07-01",
            "end": "2000-11-25",
            "institution": "Campbell, Delgado and Parker",
            "city": "West William",
            "subentries": [
                {
                    "beginning": "1977-07-01",
                    "end": "1980-07-01",
                    "description": {
                        "context": "first part"
                    }
                },
                {
                    "beginning": "1980-07-01",
                    "end": "1987-07-01",
                    "description": {
                        "context": "second part"
                    }
                }
            ]
        }
        "#;
        let entry: CVEntry = serde_json::from_str(&data).unwrap();
        let tex = entry.to_latex();
        assert_eq!(
            tex.chars().filter(|&x| x == '{').count(),
            tex.chars().filter(|&x| x == '}').count()
        );
        assert!(tex.chars().filter(|&x| x == '{').count() > 6);

        let re = Regex::new("cventry").unwrap();
        assert!(re.captures_iter(&tex).collect::<Vec<_>>().len() > 2);
    }
}
