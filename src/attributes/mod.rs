pub mod global;

#[derive(Clone)]
pub enum Attribute {
    ThAttribute(ThAttribute),
}

#[derive(Clone)]
pub enum ThAttribute {
    Abbr(String),
    /// Values higher than 1_000 are commonly ignored
    Colspan(u16),
    Headers(Vec<String>),
    Rowspan(u16),
    Scope(Scope),
}

#[derive(Clone)]
pub enum Scope {
    Row,
    Col,
    Rowgroup,
    Colgroup,
}
