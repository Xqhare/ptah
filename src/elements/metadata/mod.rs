use base::Base;
use link::Link;
use title::Title;

pub mod base;
pub mod link;
pub mod title;

#[derive(Clone, PartialEq)]
pub enum MetadataContent {
    /// Head only
    Base(Base),
    /// Phrasing content only if `itemprop` attribute is present
    Link(Link),
    /// Flow and phrasing content only if `itemprop` attribute is present
    Meta,
    /// Can be freely used as `Metadata` and `Phrasing`
    Template,
    /// Head only
    Title(Title),
}
