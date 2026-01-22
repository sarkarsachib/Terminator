pub struct AutonomyPolicy {
    pub allow_self_initiated_actions: bool,
}

impl Default for AutonomyPolicy {
    fn default() -> Self {
        Self {
            allow_self_initiated_actions: false,
        }
    }
}
