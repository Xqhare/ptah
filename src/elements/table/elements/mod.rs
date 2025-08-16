use crate::{attributes::global::GlobalAttribute, elements::FlowContent, error::{table::TableError, PtahError, Result}};

#[derive(Clone)]
pub enum TableElement {
    Caption(Caption),
    Col,
    ColGroup,
    TBody,
    Td,
    TFoot,
    Th,
    THead,
    Tr,
}

/// Category: None
/// Contexts: First child of Table
/// Content model: Flow with no Table children
/// Tag omission: Allowed for end tag if not directly followed by whitespace or comment
/// Content attributes: Global
/// Spec: https://html.spec.whatwg.org/multipage/tables.html#the-caption-element
///
/// Represents the title of a table that is its parent.
/// Introduces context for a table - making it easier to understand.
/// If the parent table is a child of a figure element, the caption element should be omitted in
/// favor of a figcaption element.
#[derive(Clone)]
pub struct Caption {
    global_attributes: Vec<GlobalAttribute>,
    content: Vec<FlowContent>,
}

impl Default for Caption {
    fn default() -> Self {
        Caption {
            global_attributes: vec![],
            content: vec![],
        }
    }
}

impl Caption {
    pub fn new() -> Caption {
        Caption::default()
    }

    pub fn add_content(&mut self, content: FlowContent) -> Result<()> {
        match content {
            FlowContent::Table(_) => Err(PtahError::TableError(TableError::InvalidTableCaptionChild)),
            _ => {
                self.content.push(content);
                Ok(())
            },
        }
    }

    pub fn add_global_attribute(&mut self, attribute: GlobalAttribute) {
        self.global_attributes.push(attribute);
    }

    pub fn get_content(&self) -> Vec<FlowContent> {
        self.content.clone()
    }

    pub fn get_global_attributes(&self) -> Vec<GlobalAttribute> {
        self.global_attributes.clone()
    }
}
