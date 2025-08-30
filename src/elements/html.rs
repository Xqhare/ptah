use crate::attributes::global::GlobalAttribute;

use super::{body::Body, head::Head};


/// The root element
#[derive(Clone)]
pub struct Html {
    global_attributes: Vec<GlobalAttribute>,
    lang: Option<String>,
    head: Option<Head>,
    body: Option<Body>,
}

impl Html {
    pub fn new(lang: Lang) -> Html {
        Html {
            global_attributes: vec![],
            lang: None,
            head: None,
            body: None,
        }
    }
}
