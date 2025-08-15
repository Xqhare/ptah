use table::Table;

pub mod table;

pub enum TableElement {
    Caption,
    Col,
    ColGroup,
    TBody,
    Td,
    TFoot,
    Th,
    THead,
    Tr,
}

/// Almost everything is flow content - a bit of metadata is not
pub enum FlowContent {
    Address,
    Blockquote,
    Dialog,
    Div,
    Dl,
    Fieldset,
    Figure,
    Footer,
    Form,
    Header,
    Hr,
    Interactive(InteractiveContent),
    Main,
    Menu,
    Ol,
    P,
    Phrasing(PhrasingContent),
    Pre,
    Search,
    Table(Table),
    Ul,
    Heading(HeadingContent),
    Sectioning(SectioningContent),
}

/// All text of the document
pub enum PhrasingContent {
    Abbr,
    /// Only for descendants of a `Map` element
    Area,
    B,
    Bdi,
    Bdo,
    Br,
    Cite,
    Code,
    Data,
    DataList,
    Del,
    Dfn,
    Em,
    Embedded(EmbeddedContent),
    I,
    Ins,
    Kbd,
    Map,
    Mark,
    Meter,
    /// NOT IMPLEMENTED IN PTAH - see README
    Noscript,
    Output,
    Progress,
    Q,
    Ruby,
    S,
    Samp,
    /// NOT IMPLEMENTED IN PTAH - see README
    Script,
    /// Only if it is a child of a `Button` inside a `Select`
    SelectedContent,
    Slot,
    Small,
    Span,
    Strong,
    Sub,
    Sup,
    Time,
    U,
    Var,
    Wbr,
    /// Literally ASCII text + whitespace only, raw dogging in the file
    Text,
}

pub enum MetadataContent {
    /// Head only
    Base,
    /// Phrasing content only if `itemprop` attribute is present
    Link,
    /// Flow and phrasing content only if `itemprop` attribute is present
    Meta,
    /// Can be freely used as `Metadata` and `Phrasing`
    Template,
    /// Head only
    Title,
}

pub enum InteractiveContent {
    /// May have no `InteractiveContent` or `A`-element children
    /// nor may any child have the `tabindex` attribute specified
    A,
    /// Without `controls` attribute it is Non-Interactive
    Audio,
    Button,
    Details,
    Embed,
    Iframe,
    /// Without `usemap` attribute it is Non-Interactive
    Img,
    /// If `type` attribute is in the `Hidden` state it is Non-Interactive
    Input,
    Label,
    Select,
    Textarea,
    /// Without `controls` attribute it is Non-Interactive
    Video,
}

pub enum EmbeddedContent {
    Canvas,
    MathML,
    Object,
    Picture,
    SVG,
}

/// Content that defines heading of a section
pub enum HeadingContent {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    /// Only if any `H1` to `H6` is a child
    HGroup,
}

/// Content that defines the scope of `header` and `footer` elements
pub enum SectioningContent {
    Article,
    Aside,
    Nav,
    Section,
}
