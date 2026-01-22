pub enum OutputMode {
    Silent,
    Conditional,
    VoiceFirst,
}

pub struct OutputSignal {
    pub requested: bool,
    pub critical: bool,
    pub confidence: f32,
}

pub fn should_emit(mode: OutputMode, signal: OutputSignal, threshold: f32) -> bool {
    match mode {
        OutputMode::Silent => signal.requested,
        OutputMode::VoiceFirst => true,
        OutputMode::Conditional => signal.requested || signal.critical || signal.confidence >= threshold,
    }
}
