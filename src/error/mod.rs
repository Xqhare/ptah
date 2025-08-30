
pub mod base;
pub mod head;
pub mod table;
pub mod title;

pub type Result<T> = std::result::Result<T, PtahError>;

#[derive(Debug)]
pub enum PtahError {
    BaseError(base::BaseError),
    HeadError(head::HeadError),
    TableError(table::TableError),
    TitleError(title::TitleError),
}

