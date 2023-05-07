#[derive(Debug, Clone)]
pub struct Entry {
    pub first_chunk: u32,
    pub samples_per_chunk: u32,
    pub sample_description_index: u32,
}
