use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminatorConfig {
    pub core: CoreConfig,
    pub output: OutputConfig,
    pub personality: PersonalityConfig,
}

impl Default for TerminatorConfig {
    fn default() -> Self {
        Self {
            core: CoreConfig::default(),
            output: OutputConfig::default(),
            personality: PersonalityConfig::default(),
        }
    }
}

impl TerminatorConfig {
    pub fn load_from_repo_defaults() -> Option<Self> {
        let path = std::path::Path::new("config/architecture.toml");
        let raw = std::fs::read_to_string(path).ok()?;
        toml::from_str(&raw).ok()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    pub event_log_capacity: usize,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            event_log_capacity: 10_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub mode: OutputMode,
    pub confidence_threshold: f32,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            mode: OutputMode::Silent,
            confidence_threshold: 0.75,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputMode {
    Silent,
    Conditional,
    VoiceFirst,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityConfig {
    pub disorder_intensity: f32,
    pub irrationality_trigger_rate: f32,
}

impl Default for PersonalityConfig {
    fn default() -> Self {
        Self {
            disorder_intensity: 0.1,
            irrationality_trigger_rate: 0.05,
        }
    }
}
