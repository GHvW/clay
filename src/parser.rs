pub trait Parser {
    type Out;

    fn call(&self, bytes: &[u8]) -> Option<(Self::Out, &[u8])>;
}