use crate::config::{OutputConfig, OutputMode};

#[derive(Debug, Clone)]
pub struct OutputSignal {
    pub user_requested: bool,
    pub critical: bool,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub enum OutputDecision {
    Suppress,
    Emit,
}

#[derive(Debug, Clone)]
pub struct GatingEngine {
    config: OutputConfig,
}

impl GatingEngine {
    pub fn new(config: OutputConfig) -> Self {
        Self { config }
    }

    pub fn decide(&self, signal: &OutputSignal) -> OutputDecision {
        match self.config.mode {
            OutputMode::Silent => {
                if signal.user_requested {
                    OutputDecision::Emit
                } else {
                    OutputDecision::Suppress
                }
            }
            OutputMode::VoiceFirst => OutputDecision::Emit,
            OutputMode::Conditional => {
                if signal.user_requested || signal.critical {
                    return OutputDecision::Emit;
                }

                if signal.confidence >= self.config.confidence_threshold {
                    OutputDecision::Emit
                } else {
                    OutputDecision::Suppress
                }
            }
        }
    }
}
