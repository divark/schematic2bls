pub mod extraction;
pub mod mapping;

#[derive(PartialEq, Debug, Clone)]
pub struct LargestCube {
    pub side_length: u16,
    pub indexes: (usize, usize, usize),
}
