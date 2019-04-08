//! Types.

use std::collections::{HashMap, HashSet};

use failure::{bail, Fallible};
use serde::{Deserialize, Serialize};

/// Works.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Works {
    /// Works.
    pub works: HashMap<WorkId, Work>,
    /// Author.
    pub authors: HashMap<AuthorId, Author>,
}

impl Works {
    /// Validete.
    pub fn validate(&self) -> Fallible<()> {
        let valid_work_ids = self.works.keys().cloned().collect::<HashSet<_>>();
        let valid_author_ids = self.authors.keys().cloned().collect::<HashSet<_>>();

        for (work_id, work) in &self.works {
            for author_id in &work.authors {
                if !valid_author_ids.contains(author_id) {
                    bail!("Unknown author ID: {:?} (work_id={:?})", author_id, work_id);
                }
            }
            for reference in &work.references {
                if !valid_work_ids.contains(&reference.work) {
                    bail!(
                        "Unknown work ID: {:?} (work_id={:?})",
                        reference.work,
                        work_id
                    );
                }
            }
        }

        Ok(())
    }

    /// Writes dot file to the given writer.
    pub fn write_graph(&self, mut writer: impl std::io::Write) -> Fallible<()> {
        writeln!(writer, "digraph works {{")?;
        writeln!(writer, "node [")?;
        writeln!(writer, r#"margin = "0.5,0.15","#)?;
        writeln!(writer, "]")?;
        for (work_id, work) in &self.works {
            writeln!(writer, r#""{}" ["#, work_id.to_dot_name())?;
            writeln!(writer, "shape = box,")?;
            if let Some(url) = work.urls.first() {
                writeln!(writer, r#"URL = "{}","#, url)?;
            }
            writeln!(
                writer,
                r#"label = "{}\n{}","#,
                work.title,
                work_id.to_dot_name()
            )?;
            writeln!(writer, "];")?;
            for reference in &work.references {
                writeln!(
                    writer,
                    r#""{}" -> "{}";"#,
                    work_id.to_dot_name(),
                    reference.work.to_dot_name()
                )?;
            }
        }
        writeln!(writer, "}}")?;

        Ok(())
    }
}

/// Work ID.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkId(pub String);

impl WorkId {
    /// Returns name usable in dot data.
    pub fn to_dot_name(&self) -> &str {
        &self.0
    }
}

/// Work.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Work {
    /// Title.
    pub title: String,
    /// Authors string.
    pub authors_string: String,
    /// Media.
    pub media: Option<Media>,
    /// Pages.
    pub pages: Option<String>,
    /// Year.
    pub year: Option<i32>,
    /// Month.
    pub month: Option<i32>,
    /// Authors.
    pub authors: Vec<AuthorId>,
    /// Other works the work refers.
    #[serde(default)]
    pub references: Vec<Reference>,
    /// URLs.
    #[serde(default)]
    pub urls: Vec<String>,
}

/// Media.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Media {
    /// Media title.
    pub title: Option<String>,
    /// Media organization.
    pub organization: Option<String>,
    /// Number.
    pub number: Option<String>,
}

/// Author ID.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AuthorId(pub String);

impl AuthorId {
    /// Returns name usable in dot data.
    pub fn to_dot_name(&self) -> &str {
        &self.0
    }
}

/// Author.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Author {
    /// Name.
    pub name: String,
}

/// Reference.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Reference {
    /// Work ID.
    pub work: WorkId,
    /// Media title.
    pub media_title: Option<String>,
    /// Authors string.
    pub authors_string: Option<String>,
}
