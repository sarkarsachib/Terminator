pub fn reason(facts: &[&str]) -> Vec<String> {
    facts.iter().map(|s| s.to_string()).collect()
}
