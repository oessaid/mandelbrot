#[derive(Debug, Clone)]
pub struct WriteImageError;
impl std::fmt::Display for WriteImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to write image to file.")
    }
}

impl From<std::io::Error> for WriteImageError {
    fn from(_: std::io::Error) -> Self {
        WriteImageError {}
    }
}

impl From<image::ImageError> for WriteImageError {
    fn from(_: image::ImageError) -> Self {
        WriteImageError {}
    }
}
