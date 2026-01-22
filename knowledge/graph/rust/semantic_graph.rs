pub struct Node {
    pub id: String,
}

pub struct Edge {
    pub from: String,
    pub to: String,
    pub weight: f32,
}

pub struct SemanticGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl SemanticGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}
