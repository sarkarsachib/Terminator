pub struct ContextFrame {
    pub payload: Vec<u8>,
}

pub fn aggregate(frames: Vec<ContextFrame>) -> ContextFrame {
    let mut out = Vec::new();
    for f in frames {
        out.extend_from_slice(&f.payload);
        out.push(b'\n');
    }
    ContextFrame { payload: out }
}
