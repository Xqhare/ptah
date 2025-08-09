use crate::attributes::global::GlobalAttribute;


pub struct Caption {
    global_attributes: Vec<GlobalAttribute>,
    caption: String,
}

impl Caption {
    pub fn new(caption: String) -> Caption {
        Caption {
            global_attributes: vec![],
            caption,
        }
    }

    pub fn add_global_attribute(&mut self, attribute: GlobalAttribute) {
        self.global_attributes.push(attribute);
    }
}
