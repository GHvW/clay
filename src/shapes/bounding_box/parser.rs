use crate::parser::Parser;
use crate::shapes::bounding_box::shape::BoundingBox;

pub struct BoundingBoxParser {

}

impl Parser for BoundingBoxParser {
    type Out = BoundingBox;

    fn call(&self, bytes: &[u8]) -> Option<(Self::Out, &[u8])> {
        Some((BoundingBox::new(10.0, 10.0, 10.0, 10.0), &Vec::new()))
    }
}