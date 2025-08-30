use crate::{attributes::global::GlobalAttribute, error::{title::TitleError, PtahError, Result}};


/// Category: Meta
/// Contexts: Head with no other title
/// Content model: Text that is not whitespace only
/// Tag omission: No tag omissible
/// Content attributes: Global
/// Spec: https://html.spec.whatwg.org/multipage/semantics.html#the-title-element
///
/// Represents the document's title or name used to identify when the document is used out of
/// context (e.g. in bookmarks).
#[derive(Clone, PartialEq)]
pub struct Title {
    global_attributes: Vec<GlobalAttribute>,
    content: String,
}

impl Title {
    pub fn new(content: String) -> Result<Title> {
        if content.is_empty() || content.trim().is_empty() {
            Err(PtahError::TitleError(TitleError::EmptyTitle))
        } else {
            Ok(Title {
                global_attributes: vec![],
                content,
            })
        }
    }

    pub fn add_global_attribute(&mut self, attribute: GlobalAttribute) {
        self.global_attributes.push(attribute);
    }

    pub fn get_global_attributes(&self) -> Vec<GlobalAttribute> {
        self.global_attributes.clone()
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn set_content(&mut self, content: String) -> Result<()> {
        if content.is_empty() || content.trim().is_empty() {
            Err(PtahError::TitleError(TitleError::EmptyTitle))
        } else {
            self.content = content;
            Ok(())
        }
    }
}
