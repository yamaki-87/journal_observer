use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum JournalUnit {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
}

impl JournalUnit {
    pub fn as_str(&self) -> &str {
        match self {
            JournalUnit::System => "_SYSTEMD_UNIT",
            JournalUnit::User => "_SYSTEMD_USER_UNIT",
        }
    }

    pub fn from_str(name: &str) -> JournalUnit {
        match name {
            "system" => JournalUnit::System,
            "user" => JournalUnit::User,
            _ => panic!("Unknown unit type: {}", name),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct JournalConfig {
    pub services: Vec<JournalObserver>,
}

#[derive(Debug, Deserialize)]
pub struct JournalObserver {
    name: String,
    #[serde(rename = "type")]
    unit_type: JournalUnit,
    keywords: Vec<String>,
}

impl JournalObserver {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_unit_type(&self) -> JournalUnit {
        self.unit_type
    }

    pub fn keywords_contain(&self, target: &str) -> bool {
        self.keywords.iter().any(|kw| target.contains(kw))
    }
}

pub fn load_config(path: &str) -> Result<JournalConfig> {
    let contents = std::fs::read_to_string(path)?;
    let targets: JournalConfig = serde_yaml::from_str(&contents)?;
    Ok(targets)
}
