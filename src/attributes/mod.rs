pub mod global;
pub enum Attribute {
    ThAttribute(ThAttribute),
}

pub enum ThAttribute {
    Abbr(String),
    /// Values higher than 1_000 are commonly ignored
    Colspan(u16),
    Headers(Vec<String>),
    Rowspan(u16),
    Scope(Scope),
}

pub enum Scope {
    Row,
    Col,
    Rowgroup,
    Colgroup,
}
