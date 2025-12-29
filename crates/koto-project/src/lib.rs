//! Koto Project - Project management

use koto_core::{SampleRate, Tempo, TimeSignature};
use koto_timeline::Timeline;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub author: String,
    pub description: String,
    pub created: String,
    pub modified: String,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            name: "Untitled".to_string(),
            author: String::new(),
            description: String::new(),
            created: String::new(),
            modified: String::new(),
        }
    }
}

/// Project file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub metadata: ProjectMetadata,
    pub sample_rate: SampleRate,
    pub tempo: Tempo,
    pub time_signature: TimeSignature,
    pub timeline: Timeline,
    #[serde(skip)]
    pub path: Option<PathBuf>,
    #[serde(skip)]
    pub modified: bool,
}

impl Project {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            metadata: ProjectMetadata {
                name: name.into(),
                ..Default::default()
            },
            sample_rate: SampleRate::default(),
            tempo: Tempo::DEFAULT,
            time_signature: TimeSignature::COMMON_TIME,
            timeline: Timeline::new(),
            path: None,
            modified: false,
        }
    }

    /// Save project to file
    pub fn save(&mut self, path: PathBuf) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(&path, json)?;
        self.path = Some(path);
        self.modified = false;
        Ok(())
    }

    /// Load project from file
    pub fn load(path: PathBuf) -> Result<Self, std::io::Error> {
        let json = std::fs::read_to_string(&path)?;
        let mut project: Project = serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        project.path = Some(path);
        project.modified = false;
        Ok(project)
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new("Untitled")
    }
}
