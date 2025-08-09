use crate::attributes::global::GlobalAttribute;

use super::Element;

pub mod caption;

pub struct Table {
    global_attributes: Vec<GlobalAttribute>,
    children: Vec<Element>,
}

impl Default for Table {
    fn default() -> Self {
        Table {
            global_attributes: vec![],
            children: vec![],
        }
    }
}

impl Table {
    pub fn new() -> Table {
        Table::default()
    }

    pub fn add_global_attribute(&mut self, attribute: GlobalAttribute) {
        self.global_attributes.push(attribute);
    }

    pub fn add_child(&mut self, child: Element) {
        if self.allowed_child(&child) {
            self.children.push(child);
        }
    }

    fn allowed_child(&self, child: &Element) -> bool {
        match child {
            Element::Caption => {
                if self.children.is_empty() {
                    true
                } else {
                    false
                }
            },
            Element::Col => true,
            Element::Colgroup => true,
            Element::Tbody => true,
            Element::Td => true,
            Element::Tfoot => true,
            Element::Th => true,
            Element::Thead => true,
            Element::Tr => true,
            _ => false,
        }
    }
}
