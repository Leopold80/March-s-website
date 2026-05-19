#[derive(Debug, Clone)]
pub struct MediaItem {
    pub filename: String,
    pub media_type: MediaType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MediaType {
    Photo,
    Video,
}
