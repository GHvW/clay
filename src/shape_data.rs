// use crate::main_file_header::MainFileHeader;
// use crate::record_header::RecordHeader;

// pub struct ShapeData<A> {
//     pub file_header: MainFileHeader,
//     pub shapes: Vec<(RecordHeader, A)>
// }

// impl<A> ShapeData<A> {
//     pub fn new(file_header: MainFileHeader, shapes: Vec<(RecordHeader, A)>) -> Self {
//         Self {
//             file_header,
//             shapes
//         }
//     }
// }


// pub struct ShapeDataBuilder<A> {
//     file_header: Option<MainFileHeader>,
//     shapes: Option<Vec<(RecordHeader, A)>>
// }

// impl<A> ShapeDataBuilder<A> {
//     pub fn new() -> Self {
//         Self { file_header: None, shapes: None }
//     }

//     pub fn file_header(self, header: MainFileHeader) -> Self {
//         Self {
//             file_header: Some(header), 
//             ..self
//         }
//     }

//     pub fn shapes(self, shapes: Vec<(RecordHeader, A)>) -> Self {
//         Self {
//             shapes: Some(shapes), 
//             ..self
//         }
//     }

//     // pub fn build(self) -> ShapeData<A> {

//     //     ShapeData::new()
//     // }
// }