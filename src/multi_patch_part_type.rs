
pub enum MultiPatchPartType {
    TriangleStrip,
    TriangleFan,
    OuterRing,
    InnerRing,
    FirstRing,
    Ring
}


pub fn resolve_multi_patch_part_type(n: i32) -> Option<MultiPatchPartType> {
    match n {
        0 => Some(MultiPatchPartType::TriangleStrip),
        1 => Some(MultiPatchPartType::TriangleFan),
        2 => Some(MultiPatchPartType::OuterRing),
        3 => Some(MultiPatchPartType::FirstRing),
        5 => Some(MultiPatchPartType::Ring),
        _ => None
    }
}