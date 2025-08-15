use crate::attributes::global::GlobalAttribute;

use super::TableElement;

pub mod caption;

pub struct Table {
    global_attributes: Vec<GlobalAttribute>,
    children: Vec<TableElement>,
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

    pub fn add_child(&mut self, child: TableElement) {
        if self.allowed_child(&child) {
            self.children.push(child);
        }
    }

    #[allow(unreachable_patterns)]
    fn allowed_child(&self, child: &TableElement) -> bool {
        match child {
            TableElement::Caption => {
                if self.children.is_empty() {
                    true
                } else {
                    false
                }
            },
            TableElement::Col => true,
            TableElement::ColGroup => true,
            TableElement::TBody => true,
            TableElement::Td => true,
            TableElement::TFoot => true,
            TableElement::Th => true,
            TableElement::THead => true,
            TableElement::Tr => true,
            _ => false,
        }
    }
}
