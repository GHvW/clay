
pub enum MultiPatchParttype {
    TriangleStrip,
    TriangleFan,
    OuterRing,
    InnerRing,
    FirstRing,
    Ring
}


pub fn resolveMultiPatchPartType(n: i32) -> Option<MultiPatchParttype> {
    match n {
        0 => Some(MultiPatchParttype::TriangleStrip),
        1 => Some(MultiPatchParttype::TriangleFan),
        2 => Some(MultiPatchParttype::OuterRing),
        3 => Some(MultiPatchParttype::FirstRing),
        5 => Some(MultiPatchParttype::Ring),
        _ => None
    }
}