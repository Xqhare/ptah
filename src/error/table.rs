#[derive(Debug)]
pub enum TableError {
    InvalidTableChild,
    InvalidTableCaptionChild,
}

impl std::fmt::Display for TableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableError::InvalidTableChild => write!(f, "Invalid table child"),
            TableError::InvalidTableCaptionChild => write!(f, "Invalid table caption child"),
        }
    }
}

impl std::error::Error for TableError {}
