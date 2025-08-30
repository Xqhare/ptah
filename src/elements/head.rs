use crate::{attributes::global::GlobalAttribute, error::{head::HeadError, PtahError, Result}};

use super::metadata::MetadataContent;


#[derive(Clone)]
pub struct Head {
    global_attributes: Vec<GlobalAttribute>,
    children: Vec<HeadContent>,
}

#[derive(Clone)]
pub struct HeadContent {
    // Needs at most one `title` and one `base`
    meta: Vec<MetadataContent>,
}

impl HeadContent {
    pub fn new() -> HeadContent {
        HeadContent { meta: vec![] }
    }

    pub fn add_meta(&mut self, meta: MetadataContent) -> Result<()> {
        match meta {
            MetadataContent::Title(_) => {
                for m in self.meta.iter() {
                    if let MetadataContent::Title(_) = m {
                        return Err(PtahError::HeadError(HeadError::HeadTitleAlreadySet));
                    }
                }
            }
            MetadataContent::Base(_) => {
                for m in self.meta.iter() {
                    if let MetadataContent::Base(_) = m {
                        return Err(PtahError::HeadError(HeadError::HeadBaseAlreadySet));
                    }
                }
            }
            _ => {},
        }
        self.meta.push(meta);
        Ok(())
    }

    pub fn get_meta(&self) -> &Vec<MetadataContent> {
        &self.meta
    }
}


