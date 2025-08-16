use crate::{attributes::global::GlobalAttribute, error::{base::BaseError, PtahError, Result}};


/// Category: Meta
/// Contexts: Head with no other base
/// Content model: Empty
/// Tag omission: No end tag
/// Content attributes: Global, href, target
/// Spec: https://html.spec.whatwg.org/multipage/semantics.html#the-base-element
///
/// Allows specification of the document base url, the name of the default navigable.
/// Either the `href` or the `target` attribute must be present or both
#[derive(Clone)]
pub struct Base {
    global_attributes: Vec<GlobalAttribute>,
    href: Option<String>,
    target: Option<String>,
}

impl Base {
    pub fn new(href: Option<String>, target: Option<String>) -> Result<Base> {
        if href.is_none() && target.is_none() {
            return Err(PtahError::BaseError(BaseError::NeitherHrefNorTargetPresent));
        } else {
            Ok(Base {
                global_attributes: vec![],
                href,
                target,
            })
        }
    }

    pub fn add_global_attribute(&mut self, attribute: GlobalAttribute) {
        self.global_attributes.push(attribute);
    }

    pub fn get_global_attributes(&self) -> Vec<GlobalAttribute> {
        self.global_attributes.clone()
    }

    pub fn get_target(&self) -> Option<String> {
        self.target.clone()
    }

    pub fn get_href(&self) -> Option<String> {
        self.href.clone()
    }

    pub fn set_target(&mut self, target: String) {
        self.target = Some(target);
    }

    pub fn set_href(&mut self, href: String) {
        self.href = Some(href);
    }
}
