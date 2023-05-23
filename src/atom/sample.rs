#[derive(Debug, Clone)]
pub struct Sample {
    pub duration: Option<u32>,
    pub size: Option<u32>,
    pub flags: Option<u32>,
    pub composition_time_offset: Option<i32>,
    pub description_index: Option<u32>,
}
