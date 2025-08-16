pub mod table;
pub mod base;

pub type Result<T> = std::result::Result<T, PtahError>;

#[derive(Debug)]
pub enum PtahError {
    BaseError(base::BaseError),
    TableError(table::TableError),
}

